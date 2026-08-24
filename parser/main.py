import time
from fastapi import FastAPI, File, UploadFile, HTTPException, status
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse

from pydantic import BaseModel
from typing import Optional, List, Dict, Any

from services.extractor import extract_document_text
from services.ner import extract_entities_and_clauses
from services.rag_engine import extract_rag_compliance_facts, anonymize_text, proclamation_corpus
from services.ai_extractor import extract_ai_compliance_facts

app = FastAPI(
    title="Kagua Ethiopian Labour Proclamation Parsing & RAG Service",
    description="FastAPI service for in-memory document parsing, spaCy NER, and privacy-preserving RAG compliance extraction against Ethiopian Labour Proclamation No. 1156/2019.",
    version="2.0.0"
)

class AIExtractRequest(BaseModel):
    anonymized_text: str
    provider: str
    model: str
    api_key: str
    proclamation_articles: Optional[List[Dict[str, Any]]] = None
    domain: Optional[str] = "auto"

# Enable CORS for frontend integration
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

MAX_FILE_SIZE_BYTES = 20 * 1024 * 1024  # 20MB limit

@app.get("/")
def read_root():
    return {
        "service": "Kagua Ethiopian Proclamation Legal Parser & RAG Engine API",
        "supported_domains": ["ethiopian_labour_proclamation", "commercial_code"],
        "status": "online",
        "endpoints": {
            "parse": "POST /parse (Upload document for Ethiopian Labour or Commercial Code NER & RAG fact extraction)",
            "rag_parse": "POST /rag-parse (Dedicated RAG fact extractor with domain auto-detection)",
            "ai_extract": "POST /ai-extract (LLM compliance fact extraction via Grok, Gemini, or GPT)"
        }
    }

@app.get("/health")
def health_check():
    return {"status": "ok", "timestamp": time.time()}

@app.post("/parse")
async def parse_document(
    file: UploadFile = File(...),
    domain: Optional[str] = "auto"
):
    """
    POST /parse endpoint:
    1. Streams file into RAM (0 disk writes).
    2. Extracts raw text with pypdf / python-docx.
    3. Runs spaCy NER & Legal Proclamation clause matcher.
    4. Executes Privacy-Preserving Ethiopian Legal RAG Pipeline (Labour / Commercial Code):
       - De-identifies sensitive user PII.
       - Retrieves matched Articles from legal JSON corpora.
       - Synthesizes structured Prolog facts for reasoning.
    """
    if not file or not file.filename:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="No file uploaded or filename missing."
        )

    try:
        contents = await file.read()
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail=f"Failed to read uploaded file stream: {str(e)}"
        )

    if len(contents) == 0:
        raise HTTPException(
            status_code=status.HTTP_400_BAD_REQUEST,
            detail="Uploaded file is empty (0 bytes)."
        )

    if len(contents) > MAX_FILE_SIZE_BYTES:
        raise HTTPException(
            status_code=status.HTTP_413_REQUEST_ENTITY_TOO_LARGE,
            detail=f"File size exceeds maximum threshold of {MAX_FILE_SIZE_BYTES // (1024 * 1024)}MB."
        )

    # 1. Extract raw text in-memory (returns (text, ocr_used) tuple)
    raw_text, ocr_used = extract_document_text(file.filename, file.content_type, contents)

    if not raw_text or not raw_text.strip():
        raise HTTPException(
            status_code=status.HTTP_422_UNPROCESSABLE_ENTITY,
            detail="Could not extract readable text from document."
        )

    # 2. Run spaCy NER & Clause Matcher
    ner_result = extract_entities_and_clauses(raw_text)

    # 3. Determine target domain
    target_domain = domain
    if not target_domain or target_domain == "auto":
        target_domain = ner_result["summary"].get("suggested_domain", "ethiopian_labour_proclamation")

    # 4. Run Privacy-Preserving Legal RAG Engine
    rag_result = extract_rag_compliance_facts(raw_text, domain=target_domain)

    # 5. Construct response payload
    response_payload = {
        "filename": file.filename,
        "content_type": file.content_type,
        "text_length": len(raw_text),
        "raw_text": raw_text,
        "ocr_used": ocr_used,
        "entities": ner_result["entities"],
        "clauses": ner_result["clauses"],
        "summary": ner_result["summary"],
        "rag_facts": rag_result["prolog_facts"],
        "fact_provenance": rag_result.get("fact_provenance", {}),
        "pii_redacted_count": rag_result["pii_redacted_count"],
        "retrieved_chunks": rag_result["retrieved_chunks"],
        "proclamation_metadata": rag_result["proclamation_metadata"]
    }

    return JSONResponse(status_code=200, content=response_payload)

@app.post("/rag-parse")
async def rag_parse_document(
    file: UploadFile = File(...),
    domain: Optional[str] = "auto"
):
    """
    POST /rag-parse endpoint:
    Dedicated RAG endpoint returning anonymized text, PII redaction stats, retrieved proclamation chunks,
    and structured Prolog facts.
    """
    if not file or not file.filename:
        raise HTTPException(status_code=400, detail="No file uploaded.")

    contents = await file.read()
    raw_text, _ocr_used = extract_document_text(file.filename, file.content_type, contents)
    if not raw_text or not raw_text.strip():
        raise HTTPException(status_code=422, detail="Empty or unreadable document.")

    sanitized_text, pii_map = anonymize_text(raw_text)
    rag_result = extract_rag_compliance_facts(raw_text, domain=domain)

    return JSONResponse(status_code=200, content={
        "filename": file.filename,
        "pii_redacted_count": len(pii_map),
        "redacted_tokens": list(pii_map.values()),
        "sanitized_text_preview": sanitized_text[:300],
        "prolog_facts": rag_result["prolog_facts"],
        "retrieved_chunks": rag_result["retrieved_chunks"],
        "proclamation_metadata": rag_result["proclamation_metadata"]
    })

@app.post("/ai-extract")
async def ai_extract_facts(req: AIExtractRequest):
    """
    POST /ai-extract endpoint:
    Proxies anonymized contract text + Proclamation context to LLM provider (Grok, Gemini, or GPT),
    returning structured compliance facts and reasoning for Prolog truth checking.
    """
    try:
        res = await extract_ai_compliance_facts(
            provider=req.provider,
            model=req.model,
            api_key=req.api_key,
            anonymized_text=req.anonymized_text,
            proclamation_articles=req.proclamation_articles,
            domain=req.domain
        )
        return JSONResponse(status_code=200, content=res)
    except Exception as e:
        raise HTTPException(
            status_code=status.HTTP_500_INTERNAL_SERVER_ERROR,
            detail=f"AI fact extraction failed: {str(e)}"
        )



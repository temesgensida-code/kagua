import json
import re
import httpx
from typing import Dict, List, Any, Optional

LABOUR_SYSTEM_PROMPT = """You are Kagua's Legal Fact Extraction AI Engine, specialized in Federal Democratic Republic of Ethiopia Labour Proclamation No. 1156/2019.
Your job is to read an anonymized employment contract text and extract precise structured facts that will be verified by SWI-Prolog legal reasoning rules.

Return ONLY a valid JSON object with two keys:
1. "ai_facts": A dictionary mapping fact keys to their extracted value (number, boolean, or null).
2. "ai_reasoning": A list of objects explaining each extracted fact.

Each object in "ai_reasoning" must have:
- "fact_key": string (matching one of the known fact keys below)
- "value": number or boolean
- "confidence": float between 0.0 and 1.0
- "explanation": string explaining how this fact was derived
- "source_quote": string (exact quote from document text, or empty)

REQUIRED FACT KEYS (only extract if present or derivable from text):
- probation_days (number): Probation period in working days (e.g. 60, 90, 135)
- working_hours_per_day (number): Daily working hours (e.g. 8, 8.5, 10)
- weekly_working_hours (number): Total weekly working hours (e.g. 48, 55, 60)
- overtime_hours_per_day (number): Daily overtime hours
- overtime_hours_per_week (number): Weekly overtime hours
- notice_period_days (number): Notice period for termination in days (e.g. 15, 30, 60)
- annual_leave_days (number): Days of paid annual leave per year
- maternity_leave_days (number): Days of paid maternity leave (e.g. 120)
- sick_leave_months (number): Months of sick leave allowed (e.g. 6)
- minimum_worker_age (number): Minimum age of worker specified
- unpaid_overtime_detected (boolean): True if overtime is unpaid, included in base pay, or TOIL/banked time without statutory premium pay
- weekly_rest_denied (boolean): True if worker works 7 days a week or weekly rest day is denied
- hiring_discrimination_detected (boolean): True if hiring restricts by sex, gender, religion, marital status, etc.
- pregnancy_discrimination_detected (boolean): True if pregnant workers must resign or are excluded
- severance_forfeited (boolean): True if contract forfeits or waives statutory severance pay
- worker_pays_ppe (boolean): True if worker must buy or pay deposit for safety gear / uniform / PPE
- trade_union_prohibited (boolean): True if trade union membership is banned or penalized
- dispute_appeal_waived (boolean): True if right to appeal dismissal or access labour board/court is waived
- prohibits_labour_inspection (boolean): True if communicating with government labour inspector is forbidden
- annual_leave_denied_initial_years (boolean): True if annual leave is deferred/denied during initial years
- maternity_leave_denied (boolean): True if paid maternity leave is denied or forced unpaid
- routine_rest_day_work_detected (boolean): True if Sunday / rest day work is routine rather than emergency
- max_wage_deduction_percent (number): Maximum wage deduction percentage allowed
- final_settlement_days (number): Days allowed for final settlement payment after termination

CRITICAL RULE: DO NOT HALLUCINATE FACTS. ONLY EXTRACT FACTS SUPPORTED BY THE TEXT. RETURN RAW VALID JSON ONLY."""

COMMERCIAL_SYSTEM_PROMPT = """You are Kagua's Commercial Code Fact Extraction AI Engine, specialized in Federal Democratic Republic of Ethiopia Commercial Code Proclamation No. 1243/2021.
Your job is to read an anonymized commercial document (articles of association, memorandum of association, corporate resolution, or agreement) and extract precise structured facts that will be verified by SWI-Prolog legal reasoning rules.

Return ONLY a valid JSON object with two keys:
1. "ai_facts": A dictionary mapping fact keys to their extracted value (number, string, boolean, or null).
2. "ai_reasoning": A list of objects explaining each extracted fact.

Each object in "ai_reasoning" must have:
- "fact_key": string (matching one of the known fact keys below)
- "value": number, string, or boolean
- "confidence": float between 0.0 and 1.0
- "explanation": string explaining how this fact was derived
- "source_quote": string (exact quote from document text, or empty)

REQUIRED FACT KEYS (only extract if present or derivable from text):
- company_type (string): "share_company" or "plc" or "one_person_company"
- company_capital (number): Total registered / fixed capital in ETB / Birr (e.g. 50000, 15000, 30000)
- share_par_value (number): Par value per share in ETB / Birr (e.g. 100)
- founders_count (number): Number of founders / subscribers for Share Company (e.g. 5)
- plc_members_count (number): Number of members for PLC (e.g. 2 to 50)
- paid_up_capital_percent (number): Percentage of initial capital paid up (e.g. 25, 50, 100)
- board_directors_count (number): Size of the Board of Directors (e.g. 3 to 13)
- executive_directors_percent (number): Percentage of board members who are executive / managing
- non_shareholder_directors_percent (number): Percentage of board members who are non-shareholders
- has_unapproved_director_loan (boolean): True if loan/guarantee to director lacks general meeting approval
- missing_auditor_report_for_director_loan (boolean): True if director loan lacks external auditor evaluation report
- legal_reserve_transfer_percent (number): Annual transfer rate to legal reserve fund (e.g. 5)
- fictitious_dividend_distributed (boolean): True if dividends distributed without verified net profit
- dividend_payment_delay_months (number): Delay in dividend payment after resolution (e.g. 4)
- single_member_unconverted_months (number): Months single-member PLC operated without adding member or converting
- accounting_records_retention_years (number): Years accounting records are preserved (e.g. 10)
- issues_transferable_securities (boolean): True if PLC issues bonds / transferable securities
- has_contribution_in_kind (boolean): True if non-cash property contributions exist
- has_expert_valuation_report (boolean): True if independent expert valuation report attached for in-kind property
- has_external_formation_audit (boolean): True if Share Company formation is audited by external auditors

CRITICAL RULE: DO NOT HALLUCINATE FACTS. ONLY EXTRACT FACTS SUPPORTED BY THE TEXT. RETURN RAW VALID JSON ONLY."""


def normalize_fact_value(val: Any) -> Any:
    """Coerce stringified numbers or booleans into strict native Python types."""
    if val is None:
        return None
    if isinstance(val, bool):
        return val
    if isinstance(val, (int, float)):
        return val
    if isinstance(val, str):
        v_clean = val.strip().lower()
        if v_clean in ("true", "yes"):
            return True
        if v_clean in ("false", "no"):
            return False
        # Try pure numeric conversion
        try:
            if "." in val:
                return float(val)
            return int(val)
        except ValueError:
            # Match patterns like "60 days", "8.5 hours", "50,000 ETB"
            cleaned_str = val.strip().replace(",", "")
            m = re.match(r"^(\d+(?:\.\d+)?)\s*(?:days?|hours?|months?|years?|etb|birr|%)?$", cleaned_str, re.IGNORECASE)
            if m:
                num_str = m.group(1)
                return float(num_str) if "." in num_str else int(num_str)
            return val
    return val


def parse_json_response(raw_resp: str) -> Dict[str, Any]:
    """Clean markdown backticks if present, parse JSON, and normalize fact types."""
    clean = raw_resp.strip()
    if clean.startswith("```"):
        clean = re.sub(r"^```(?:json)?\n?", "", clean)
        clean = re.sub(r"\n?```$", "", clean)
        clean = clean.strip()
    
    parsed = None
    try:
        parsed = json.loads(clean)
    except Exception as e:
        # Fallback regex extraction if JSON has trailing text
        m = re.search(r"\{.*\}", clean, re.DOTALL)
        if m:
            parsed = json.loads(m.group(0))
        else:
            raise ValueError(f"Failed to parse LLM JSON response: {e}")

    if isinstance(parsed, dict) and "ai_facts" in parsed and isinstance(parsed["ai_facts"], dict):
        normalized_facts = {}
        for k, v in parsed["ai_facts"].items():
            normalized_facts[k] = normalize_fact_value(v)
        parsed["ai_facts"] = normalized_facts

    return parsed


async def extract_ai_facts_google(
    api_key: str,
    model: str,
    anonymized_text: str,
    legal_context: str,
    system_prompt: str = LABOUR_SYSTEM_PROMPT
) -> Dict[str, Any]:
    url = f"https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
    prompt_content = f"{system_prompt}\n\nANONYMIZED DOCUMENT TEXT:\n{anonymized_text}\n\nSTATUTORY LEGAL CONTEXT:\n{legal_context}"

    payload = {
        "contents": [
            {
                "parts": [{"text": prompt_content}]
            }
        ],
        "generationConfig": {
            "responseMimeType": "application/json",
            "temperature": 0.1
        }
    }

    async with httpx.AsyncClient(timeout=30.0) as client:
        resp = await client.post(url, json=payload)
        resp.raise_for_status()
        data = resp.json()
        raw_text = data["candidates"][0]["content"]["parts"][0]["text"]
        return parse_json_response(raw_text)


async def extract_ai_facts_xai(
    api_key: str,
    model: str,
    anonymized_text: str,
    legal_context: str,
    system_prompt: str = LABOUR_SYSTEM_PROMPT
) -> Dict[str, Any]:
    url = "https://api.x.ai/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }
    user_content = f"ANONYMIZED DOCUMENT TEXT:\n{anonymized_text}\n\nSTATUTORY LEGAL CONTEXT:\n{legal_context}"

    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content}
        ],
        "temperature": 0.1,
        "response_format": {"type": "json_object"}
    }

    async with httpx.AsyncClient(timeout=30.0) as client:
        resp = await client.post(url, headers=headers, json=payload)
        resp.raise_for_status()
        data = resp.json()
        raw_text = data["choices"][0]["message"]["content"]
        return parse_json_response(raw_text)


async def extract_ai_facts_openai(
    api_key: str,
    model: str,
    anonymized_text: str,
    legal_context: str,
    system_prompt: str = LABOUR_SYSTEM_PROMPT
) -> Dict[str, Any]:
    url = "https://api.openai.com/v1/chat/completions"
    headers = {
        "Authorization": f"Bearer {api_key}",
        "Content-Type": "application/json"
    }
    user_content = f"ANONYMIZED DOCUMENT TEXT:\n{anonymized_text}\n\nSTATUTORY LEGAL CONTEXT:\n{legal_context}"

    payload = {
        "model": model,
        "messages": [
            {"role": "system", "content": system_prompt},
            {"role": "user", "content": user_content}
        ],
        "temperature": 0.1,
        "response_format": {"type": "json_object"}
    }

    async with httpx.AsyncClient(timeout=30.0) as client:
        resp = await client.post(url, headers=headers, json=payload)
        resp.raise_for_status()
        data = resp.json()
        raw_text = data["choices"][0]["message"]["content"]
        return parse_json_response(raw_text)


async def extract_ai_compliance_facts(
    provider: str,
    model: str,
    api_key: str,
    anonymized_text: str,
    proclamation_articles: Optional[List[Dict[str, Any]]] = None,
    domain: Optional[str] = "auto"
) -> Dict[str, Any]:
    """
    Main entry point for AI fact extraction service.
    Proxies request to chosen provider (google, xai, openai) and normalizes output.
    """
    if not api_key or not api_key.strip():
        raise ValueError("API key is required for AI fact extraction.")

    # Determine system prompt based on domain
    dom_clean = (domain or "auto").lower()
    if dom_clean in ("commercial_code", "commercial_code_ethiopia", "ethiopian_commercial_code"):
        system_prompt = COMMERCIAL_SYSTEM_PROMPT
        default_context = "Ethiopian Commercial Code Proclamation No. 1243/2021 standard provisions."
    elif dom_clean == "auto" and ("commercial" in anonymized_text.lower() or "share company" in anonymized_text.lower() or "plc" in anonymized_text.lower()):
        system_prompt = COMMERCIAL_SYSTEM_PROMPT
        default_context = "Ethiopian Commercial Code Proclamation No. 1243/2021 standard provisions."
    else:
        system_prompt = LABOUR_SYSTEM_PROMPT
        default_context = "Ethiopian Labour Proclamation No. 1156/2019 standard provisions."

    # Build legal context summary from retrieved proclamation articles
    context_lines = []
    if proclamation_articles:
        for art in proclamation_articles:
            art_num = art.get("article_number", "")
            title = art.get("title", "")
            snippet = art.get("text", "")[:300]
            context_lines.append(f"Article {art_num} ({title}): {snippet}")
    legal_context = "\n".join(context_lines) if context_lines else default_context

    provider_lower = provider.lower()
    if provider_lower in ("google", "gemini"):
        parsed = await extract_ai_facts_google(api_key, model, anonymized_text, legal_context, system_prompt=system_prompt)
    elif provider_lower in ("xai", "grok"):
        parsed = await extract_ai_facts_xai(api_key, model, anonymized_text, legal_context, system_prompt=system_prompt)
    elif provider_lower in ("openai", "gpt"):
        parsed = await extract_ai_facts_openai(api_key, model, anonymized_text, legal_context, system_prompt=system_prompt)
    else:
        raise ValueError(f"Unsupported AI provider: '{provider}'. Must be 'google', 'xai', or 'openai'.")

    ai_facts = parsed.get("ai_facts", {})
    ai_reasoning = parsed.get("ai_reasoning", [])

    return {
        "status": "ok",
        "provider": provider,
        "model": model,
        "ai_facts": ai_facts,
        "ai_reasoning": ai_reasoning,
        "facts_extracted_count": len(ai_facts)
    }

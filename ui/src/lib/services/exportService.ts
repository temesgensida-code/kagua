import type { AnalysisReport, MappedViolation } from './api';

/**
 * Exports an AnalysisReport as a structured, pretty-printed JSON file.
 */
export function exportReportAsJson(report: AnalysisReport): void {
  const exportPayload = {
    schema_version: "2.0.0",
    export_timestamp: new Date().toISOString(),
    audit_id: report.id || null,
    filename: report.filename,
    compliance_score: report.compliance_score ?? null,
    jurisdiction: report.detected_jurisdiction || "Federal Democratic Republic of Ethiopia",
    governing_domains: report.domains_checked,
    summary: {
      total_violations: report.total_violations,
      critical_count: report.critical_count,
      warning_count: report.warning_count,
      entities_extracted: report.entities_extracted,
      clauses_detected: report.clauses_detected,
      pii_redacted_count: report.pii_redacted_count || 0,
    },
    violations: report.violations.map((v) => ({
      domain: v.domain,
      rule: v.rule,
      title: v.title,
      severity: v.severity,
      statutory_citation: v.article_citation || null,
      statutory_text: v.statutory_text || null,
      contract_snippet: v.snippet || null,
      start_char: v.start_char ?? null,
      end_char: v.end_char ?? null,
      description: v.description,
      remedial_recommendation: v.recommendation,
    })),
    extracted_facts: report.rag_facts || {},
    fact_provenance: report.fact_provenance || {},
  };

  const jsonStr = JSON.stringify(exportPayload, null, 2);
  downloadBlob(jsonStr, 'application/json', `kagua_audit_${sanitizeFilename(report.filename)}.json`);
}

/**
 * Exports all detected violations into a clean CSV spreadsheet matrix.
 */
export function exportReportAsCsv(report: AnalysisReport): void {
  const headers = [
    'Violation ID',
    'Domain',
    'Severity',
    'Rule & Article',
    'Title',
    'Description',
    'Contract Snippet',
    'Statutory Text',
    'Remedial Recommendation',
  ];

  const escapeCsv = (val: string | number | undefined | null): string => {
    if (val === undefined || val === null) return '""';
    const s = String(val).replace(/"/g, '""');
    return `"${s}"`;
  };

  const rows = (report.violations || []).map((v, idx) => [
    escapeCsv(idx + 1),
    escapeCsv(v.domain),
    escapeCsv(v.severity.toUpperCase()),
    escapeCsv(v.rule),
    escapeCsv(v.title),
    escapeCsv(v.description),
    escapeCsv(v.snippet || ''),
    escapeCsv(v.statutory_text || ''),
    escapeCsv(v.recommendation),
  ]);

  const csvContent = [headers.join(','), ...rows.map((r) => r.join(','))].join('\r\n');
  downloadBlob(csvContent, 'text/csv;charset=utf-8;', `kagua_violations_${sanitizeFilename(report.filename)}.csv`);
}

/**
 * Exports an executive Markdown legal briefing for corporate counsel and HR.
 */
export function exportReportAsMarkdown(report: AnalysisReport): void {
  const score = report.compliance_score !== undefined
    ? `${report.compliance_score.toFixed(1)} / 100`
    : 'Evaluated';

  let md = `# Kagua Legal Compliance Audit Report\n\n`;
  md += `**Document Name:** \`${report.filename}\`  \n`;
  md += `**Audit Timestamp:** ${report.created_at ? new Date(report.created_at).toUTCString() : new Date().toUTCString()}  \n`;
  md += `**Jurisdiction:** ${report.detected_jurisdiction || 'Ethiopia'}  \n`;
  md += `**Compliance Score:** **${score}**  \n`;
  md += `**Total Violations Detected:** ${report.total_violations} (${report.critical_count} Critical, ${report.warning_count} Warnings)  \n\n`;

  md += `---\n\n`;
  md += `## Executive Summary\n\n`;
  if (report.total_violations === 0) {
    md += `✅ **Fully Compliant**: No statutory violations were identified against the evaluated Ethiopian legal proclamations.\n\n`;
  } else {
    md += `⚠️ **Non-Compliance Alerts**: This document contains **${report.critical_count} critical statutory violations** and **${report.warning_count} compliance warnings** that must be remediated prior to execution.\n\n`;
  }

  md += `### Violation Details Matrix\n\n`;

  if (report.violations && report.violations.length > 0) {
    report.violations.forEach((v, i) => {
      const badge = v.severity === 'critical' ? '🔴 CRITICAL' : '🟡 WARNING';
      md += `#### ${i + 1}. [${badge}] ${v.title}\n\n`;
      md += `- **Statutory Rule:** ${v.rule}\n`;
      if (v.article_citation) {
        md += `- **Citation:** ${v.article_citation}\n`;
      }
      md += `- **Legal Finding:** ${v.description}\n`;
      if (v.snippet) {
        md += `- **Contract Language Flagged:**\n  > "${v.snippet.trim()}"\n`;
      }
      if (v.statutory_text) {
        md += `- **Statutory Standard:**\n  > ${v.statutory_text.trim()}\n`;
      }
      md += `- **Recommended Legal Action:** ${v.recommendation}\n\n`;
    });
  } else {
    md += `*No violations recorded.*\n\n`;
  }

  md += `---\n\n`;
  md += `*Generated automatically by Kagua Legal Compliance Platform (Ethiopian Proclamations Intelligence Engine).*  \n`;

  downloadBlob(md, 'text/markdown;charset=utf-8;', `kagua_briefing_${sanitizeFilename(report.filename)}.md`);
}

function sanitizeFilename(name: string): string {
  return name.replace(/\.[^/.]+$/, '').replace(/[^a-zA-Z0-9_-]/g, '_');
}

function downloadBlob(content: string, mimeType: string, filename: string): void {
  const blob = new Blob([content], { type: mimeType });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = filename;
  document.body.appendChild(a);
  a.click();
  document.body.removeChild(a);
  URL.revokeObjectURL(url);
}

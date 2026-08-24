export interface Framework {
  id: string;
  name: string;
  domain: 'labour' | 'commercial';
  color: string;
  bg: string;
  border: string;
  glow: string;
  lightBg: string;
  lightBorder: string;
  lightColor: string;
  active: boolean;
  description: string;
  ruleCount: number;
}

export const INITIAL_FRAMEWORKS: Framework[] = [
  {
    id: 'ethiopian_labour_proclamation',
    name: 'Labour Proclamation 1156/2019',
    domain: 'labour',
    color: '#CBD5E1',
    bg: '#1E293B',
    border: '#475569',
    glow: '0 0 12px rgba(148, 163, 184, 0.25)',
    lightBg: '#5B6F7A',
    lightBorder: '#3E4D56',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Federal Democratic Republic of Ethiopia Labour Proclamation No. 1156/2019',
    ruleCount: 14
  },
  {
    id: 'commercial_code_ethiopia_1243_2021',
    name: 'Commercial Code 1243/2021',
    domain: 'commercial',
    color: '#94A3B8',
    bg: '#0F172A',
    border: '#334155',
    glow: '0 0 12px rgba(100, 116, 139, 0.25)',
    lightBg: '#4A5B6C',
    lightBorder: '#334155',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Federal Democratic Republic of Ethiopia Commercial Code Proclamation No. 1243/2021',
    ruleCount: 20
  },
  {
    id: 'comm_share_company',
    name: 'Share Company (Art. 247 & 296)',
    domain: 'commercial',
    color: '#E2E8F0',
    bg: '#1E242B',
    border: '#64748B',
    glow: '0 0 12px rgba(161, 161, 170, 0.25)',
    lightBg: '#5A6778',
    lightBorder: '#3F4A58',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Share Company minimum capital (50,000 ETB), par value (100 ETB), and Board of Directors (3-13 directors)',
    ruleCount: 8
  },
  {
    id: 'comm_plc',
    name: 'PLC Governance (Art. 495 & 496)',
    domain: 'commercial',
    color: '#D1D5DB',
    bg: '#1F2937',
    border: '#4B5563',
    glow: '0 0 12px rgba(113, 113, 122, 0.25)',
    lightBg: '#4E5A65',
    lightBorder: '#36414A',
    lightColor: '#FFFFFF',
    active: true,
    description: 'PLC member limits (2-50), minimum capital (15,000 ETB), and 100% paid-up capital requirement',
    ruleCount: 6
  },
  {
    id: 'eth_probation',
    name: 'Article 11 Probation',
    domain: 'labour',
    color: '#9CA3AF',
    bg: '#111827',
    border: '#374151',
    glow: '0 0 12px rgba(156, 163, 175, 0.2)',
    lightBg: '#596573',
    lightBorder: '#3E4854',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Probation period statutory limit (Max 60 working days per Art. 11(3))',
    ruleCount: 1
  },
  {
    id: 'eth_working_hours',
    name: 'Article 61 & 67 Working Hours',
    domain: 'labour',
    color: '#A3A3A3',
    bg: '#18181B',
    border: '#3F3F46',
    glow: '0 0 12px rgba(163, 163, 163, 0.2)',
    lightBg: '#606C7A',
    lightBorder: '#45505C',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Normal working hours cap (8 hrs/day, 48 hrs/wk) & overtime cap (2 hrs/day)',
    ruleCount: 3
  },
  {
    id: 'eth_maternity',
    name: 'Article 88 Maternity Leave',
    domain: 'labour',
    color: '#A8A29E',
    bg: '#1C1917',
    border: '#44403C',
    glow: '0 0 12px rgba(168, 162, 158, 0.2)',
    lightBg: '#676C75',
    lightBorder: '#4B5058',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Paid maternity leave statutory entitlement (120 consecutive days)',
    ruleCount: 1
  },
  {
    id: 'eth_harassment',
    name: 'Article 14 Workplace Protections',
    domain: 'labour',
    color: '#94A3B8',
    bg: '#1E293B',
    border: '#334155',
    glow: '0 0 12px rgba(148, 163, 184, 0.2)',
    lightBg: '#536677',
    lightBorder: '#394957',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Anti-discrimination and zero-tolerance sexual harassment policy mandates',
    ruleCount: 2
  },
  {
    id: 'eth_severance',
    name: 'Article 39 Severance Pay',
    domain: 'labour',
    color: '#6B7280',
    bg: '#111827',
    border: '#374151',
    glow: '0 0 12px rgba(107, 114, 128, 0.2)',
    lightBg: '#4B5462',
    lightBorder: '#343B47',
    lightColor: '#FFFFFF',
    active: true,
    description: 'Severance compensation terms for lawful contract termination',
    ruleCount: 1
  }
];

export interface SampleDoc {
  id: string;
  filename: string;
  size: string;
  type: string;
  domain: 'ethiopian_labour_proclamation' | 'commercial_code' | 'auto';
  content: string;
  complianceScore: number;
  findings: Array<{
    framework: string;
    severity: 'CRITICAL' | 'WARNING' | 'PASS';
    title: string;
    description: string;
    recommendation: string;
  }>;
}

export const SAMPLE_DOCS: SampleDoc[] = [
  {
    id: 'eth_share_company_moa',
    filename: 'Ethio_Logistics_Share_Company_MoA.pdf',
    size: '1.4 MB',
    type: 'PDF',
    domain: 'commercial_code',
    content: `MEMORANDUM OF ASSOCIATION OF ETHIO LOGISTICS SHARE COMPANY

Article 1: Company Name & Legal Form
The company is established as a Share Company under the Commercial Code of Ethiopia Proclamation No. 1243/2021.

Article 2: Capital and Shares
1. The registered capital of the company is Birr 30,000 divided into 300 shares of Birr 100 each.
2. The subscribers have paid 15% of the subscribed capital at the time of formation.

Article 3: Board of Directors
1. The company shall be governed by a Board of Directors consisting of 2 members.
2. Both directors are non-shareholders and serve as executive officers.

Article 4: Reserve Fund
The company shall allocate 3% of net profits annually to the reserve fund.`,
    complianceScore: 25,
    findings: [
      {
        framework: 'Ethiopian Commercial Code Proclamation No. 1243/2021',
        severity: 'CRITICAL',
        title: 'Article 247(1) - Share Company Capital Below Statutory Minimum',
        description: 'Share Company capital (30,000 ETB) is below statutory minimum requirement of 50,000 ETB.',
        recommendation: 'Increase registered capital to at least 50,000 ETB per Article 247(1).'
      },
      {
        framework: 'Ethiopian Commercial Code Proclamation No. 1243/2021',
        severity: 'CRITICAL',
        title: 'Article 296(1) - Invalid Board of Directors Size',
        description: 'Board of Directors has 2 members, violating mandatory range of 3 to 13 directors.',
        recommendation: 'Maintain a Board of Directors with at least 3 members per Article 296(1).'
      },
      {
        framework: 'Ethiopian Commercial Code Proclamation No. 1243/2021',
        severity: 'CRITICAL',
        title: 'Article 254(1) - Insufficient Initial Paid-Up Capital',
        description: 'Paid-up capital at formation (15%) is below minimum requirement of 25%.',
        recommendation: 'Deposit at least 25% of subscribed cash capital prior to registration per Article 254(1).'
      }
    ]
  },
  {
    id: 'eth_plc_articles',
    filename: 'Abyssinia_Tech_PLC_Articles.docx',
    size: '920 KB',
    type: 'DOCX',
    domain: 'commercial_code',
    content: `ARTICLES OF ASSOCIATION OF ABYSSINIA TECH PLC

Article 1: Establishment
The company is formed as a Private Limited Company (PLC) in accordance with Proclamation No. 1243/2021.

Article 2: Capital & Shares
1. The capital of the company is Birr 10,000 divided into 100 shares of Birr 100 each.
2. 60% of the capital is paid up at formation.

Article 3: Membership
The company currently comprises 55 members.

Article 4: Reserves
Annual legal reserve transfer is set at 2% of annual net profit.`,
    complianceScore: 30,
    findings: [
      {
        framework: 'Ethiopian Commercial Code Proclamation No. 1243/2021',
        severity: 'CRITICAL',
        title: 'Article 496(1) - PLC Capital Below Statutory Minimum',
        description: 'PLC capital (10,000 ETB) is below minimum required 15,000 ETB.',
        recommendation: 'Set PLC capital to at least 15,000 ETB per Article 496(1).'
      },
      {
        framework: 'Ethiopian Commercial Code Proclamation No. 1243/2021',
        severity: 'CRITICAL',
        title: 'Article 495(4) - Excessive PLC Member Count',
        description: 'PLC member count (55) exceeds statutory cap of 50 members.',
        recommendation: 'Reduce member count to max 50 or convert to Share Company per Article 495(4).'
      },
      {
        framework: 'Ethiopian Commercial Code Proclamation No. 1243/2021',
        severity: 'CRITICAL',
        title: 'Articles 495(1) & 499(1) - Incomplete Paid-Up Capital',
        description: 'PLC capital is only 60% paid up, violating 100% advance payment requirement.',
        recommendation: 'Pay up 100% of PLC capital in full before registration per Articles 495(1) & 499(1).'
      }
    ]
  },
  {
    id: 'eth_employment_contract',
    filename: 'Ethiopia_Standard_Employment_Contract.pdf',
    size: '1.2 MB',
    type: 'PDF',
    domain: 'ethiopian_labour_proclamation',
    content: `EMPLOYMENT CONTRACT UNDER ETHIOPIAN LABOUR LAW

1. PROBATION PERIOD
The employee shall be subject to a probation period of 90 working days starting from the date of commencement.

2. WORKING HOURS & OVERTIME
Normal working hours shall be 9 hours per day (45 hours per week). Overtime work may be requested up to 3 hours per day.

3. LEAVE ENTITLEMENTS
The employee is entitled to 12 working days of annual leave. Maternity leave shall consist of 60 consecutive days with full pay. Sick leave up to 3 months is permitted with medical certification.

4. TERMINATION NOTICE
Either party may terminate this agreement by providing 15 days written notice.`,
    complianceScore: 35,
    findings: [
      {
        framework: 'Ethiopian Labour Proclamation No. 1156/2019',
        severity: 'CRITICAL',
        title: 'Article 11(3) - Excessive Probation Period',
        description: 'Specified probation period of 90 working days exceeds statutory limit of 60 working days.',
        recommendation: 'Reduce probation period to max 60 working days per Article 11(3).'
      },
      {
        framework: 'Ethiopian Labour Proclamation No. 1156/2019',
        severity: 'CRITICAL',
        title: 'Article 88(2-3) - Insufficient Maternity Leave',
        description: 'Maternity leave of 60 days is below mandatory 120 consecutive days entitlement.',
        recommendation: 'Grant 120 consecutive days paid maternity leave (30 prenatal + 90 postnatal).'
      },
      {
        framework: 'Ethiopian Labour Proclamation No. 1156/2019',
        severity: 'CRITICAL',
        title: 'Article 61(1) - Excessive Daily Working Hours',
        description: 'Daily working hours (9 hrs) exceed statutory maximum of 8 hours per day.',
        recommendation: 'Cap normal daily working hours at 8 hours per day.'
      }
    ]
  },
  {
    id: 'eth_tech_contract',
    filename: 'Addis_Tech_Senior_Developer_Agreement.docx',
    size: '850 KB',
    type: 'DOCX',
    domain: 'ethiopian_labour_proclamation',
    content: `ADDIS ABABA SOFTWARE INDUSTRY EMPLOYMENT CONTRACT

Section 1. Terms of Service & Probation
The trial probation period shall be 60 working days.

Section 2. Daily Hours
Working hours are 8 hours per day from Monday to Friday (40 hours per week).

Section 3. Annual & Sick Leave
Employee receives 16 working days of paid annual leave for the first year of service. Sick leave is granted up to 6 months per Article 85.

Section 4. Sexual Harassment & Non-Discrimination
Company maintains a zero-tolerance policy against sexual harassment and discrimination based on gender, religion, or HIV status per Article 14.`,
    complianceScore: 100,
    findings: [
      {
        framework: 'Ethiopian Labour Proclamation No. 1156/2019',
        severity: 'PASS',
        title: 'Full Proclamation No. 1156/2019 Compliance',
        description: 'All statutory clauses (Probation, Working Hours, Annual Leave, Sick Leave, Harassment Protections) fully comply with Ethiopian Labour Law.',
        recommendation: 'Maintain compliance records and annual policy review.'
      }
    ]
  }
];


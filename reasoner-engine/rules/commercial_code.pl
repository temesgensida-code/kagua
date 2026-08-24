%% ============================================================================
%% Ethiopian Commercial Code Proclamation No. 1243/2021 Compliance Rules Module
%%
%% SWI-Prolog check_violation/2 rules for verifying corporate articles of
%% association, bylaws, company formation documents, and commercial contracts
%% against the statutory provisions of Ethiopian Commercial Code Proclamation No. 1243/2021.
%% ============================================================================

:- module(commercial_code, [check_violation/2]).

:- discontiguous check_violation/2.

%% Local wrapper so unqualified doc_fact/2 calls resolve to user:doc_fact/2
doc_fact(K, V) :- user:doc_fact(K, V).

%% Helper for domain matching
ethiopian_commercial_domain(commercial_code).
ethiopian_commercial_domain(commercial_code_ethiopia).
ethiopian_commercial_domain(ethiopian_commercial_code).

%% ---------------------------------------------------------------------------
%% Article 247(1) — Share Company Minimum Capital (Min 50,000 ETB)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 247(1)',
    title:         'Share Company Capital Below Statutory Minimum',
    severity:      critical,
    description:   Desc,
    recommendation: 'Increase the fixed capital of the Share Company to at least 50,000 Ethiopian Birr (ETB) as mandated by Article 247(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, share_company),
    doc_fact(company_capital, Capital),
    number(Capital),
    Capital < 50000,
    format(atom(Desc),
           'Share Company registered capital (~w ETB) is below the statutory minimum threshold of 50,000 ETB required under Article 247(1).',
           [Capital]).

check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 247(1)',
    title:         'Share Company Capital Below Statutory Minimum',
    severity:      critical,
    description:   Desc,
    recommendation: 'Ensure Share Company capital is at least 50,000 ETB per Article 247(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(share_company_capital, Capital),
    number(Capital),
    Capital < 50000,
    format(atom(Desc),
           'Share Company capital of ~w ETB is below the statutory minimum requirement of 50,000 ETB under Article 247(1).',
           [Capital]).

%% ---------------------------------------------------------------------------
%% Article 247(2) — Share Company Minimum Share Par Value (Min 100 ETB)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 247(2)',
    title:         'Share Par Value Below Statutory Minimum',
    severity:      critical,
    description:   Desc,
    recommendation: 'Set the par value of each share to a minimum of 100 Ethiopian Birr per Article 247(2).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(share_par_value, ParValue),
    number(ParValue),
    ParValue < 100,
    format(atom(Desc),
           'Share par value (~w ETB) is less than the statutory minimum of 100 ETB per share mandated by Article 247(2).',
           [ParValue]).

%% ---------------------------------------------------------------------------
%% Article 252(2) — Share Company Minimum Number of Founders (Min 5 Founders)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 252(2)',
    title:         'Insufficient Number of Share Company Founders',
    severity:      critical,
    description:   Desc,
    recommendation: 'Ensure the Share Company has at least 5 founders/subscribers upon establishment per Article 252(2).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, share_company),
    doc_fact(founders_count, Count),
    number(Count),
    Count < 5,
    format(atom(Desc),
           'Share Company has ~w founder(s), which is fewer than the mandatory statutory minimum of 5 founders required under Article 252(2).',
           [Count]).

%% ---------------------------------------------------------------------------
%% Article 254(1) — Share Company Minimum Paid-Up Capital at Formation (Min 25%)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 254(1)',
    title:         'Insufficient Initial Paid-Up Capital for Share Company',
    severity:      critical,
    description:   Desc,
    recommendation: 'Deposit at least 25% of subscribed cash capital into a blocked bank account prior to commercial registration per Article 254(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, share_company),
    doc_fact(paid_up_capital_percent, Pct),
    number(Pct),
    Pct < 25,
    format(atom(Desc),
           'Initial paid-up capital (~w%) is below the statutory minimum requirement of 25% of subscribed cash capital under Article 254(1).',
           [Pct]).

%% ---------------------------------------------------------------------------
%% Article 296(1) — Board of Directors Size Limits (Min 3, Max 13 Directors)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 296(1)',
    title:         'Invalid Board of Directors Size',
    severity:      critical,
    description:   Desc,
    recommendation: 'Maintain a Board of Directors with between 3 and 13 members as mandated by Article 296(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(board_directors_count, Count),
    number(Count),
    ( Count < 3 ; Count > 13 ),
    format(atom(Desc),
           'Board of Directors has ~w member(s), violating the statutory range of 3 to 13 directors required under Article 296(1).',
           [Count]).

%% ---------------------------------------------------------------------------
%% Article 296(1) — Non-Executive Directors Ratio (At Least 2/3 Non-Executive)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 296(1)',
    title:         'Insufficient Non-Executive Representation on Board',
    severity:      warning,
    description:   Desc,
    recommendation: 'Ensure at least two-thirds (66.67%) of board members do not participate in day-to-day company management under Article 296(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(executive_directors_percent, ExecutivePct),
    number(ExecutivePct),
    ExecutivePct > 33.33,
    format(atom(Desc),
           'Executive directors comprise ~w% of the board, breaching the statutory cap where at least two-thirds of directors must be non-executive under Article 296(1).',
           [ExecutivePct]).

%% ---------------------------------------------------------------------------
%% Article 296(2) — Non-Shareholder Directors Limit (Max 1/3 Non-Shareholders)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 296(2)',
    title:         'Excessive Non-Shareholder Board Members',
    severity:      warning,
    description:   Desc,
    recommendation: 'Limit non-shareholder board members to a maximum of one-third (33.33%) of the total board membership per Article 296(2).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(non_shareholder_directors_percent, Pct),
    number(Pct),
    Pct > 33.33,
    format(atom(Desc),
           'Non-shareholder directors represent ~w% of the board, exceeding the maximum statutory limit of one-third (33.33%) under Article 296(2).',
           [Pct]).

%% ---------------------------------------------------------------------------
%% Article 307 — Unlawful Loans or Guarantees to Directors
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 307(1-3)',
    title:         'Unlawful Loan or Guarantee Provided to Director',
    severity:      critical,
    description:   'Company has issued a loan, guarantee, or security for a director without prior approval from the general meeting of shareholders and an independent external auditor report.',
    recommendation: 'Prohibit loans or guarantees to directors unless explicitly authorized by a shareholder general meeting following an independent auditor report under Article 307.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(has_unapproved_director_loan, true).

check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 307(3)',
    title:         'Missing Independent Auditor Report for Director Transaction',
    severity:      critical,
    description:   'Shareholder approval for a director loan or guarantee was granted without submitting an independent external auditor evaluation report.',
    recommendation: 'Obtain and submit a written report from an independent external auditor prior to voting on director financial transactions per Article 307(3).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(missing_auditor_report_for_director_loan, true).

%% ---------------------------------------------------------------------------
%% Article 434(1) — Share Company Legal Reserve Fund (Min 5% Net Profits)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 434(1)',
    title:         'Insufficient Legal Reserve Fund Transfer Rate',
    severity:      warning,
    description:   Desc,
    recommendation: 'Transfer at least 5% of net profits annually to the legal reserve fund until it reaches 5% of company capital per Article 434(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, share_company),
    doc_fact(legal_reserve_transfer_percent, Rate),
    number(Rate),
    Rate < 5,
    format(atom(Desc),
           'Annual legal reserve deduction (~w%) is below the statutory requirement of 5% of net profits under Article 434(1).',
           [Rate]).

%% ---------------------------------------------------------------------------
%% Article 438(1-2) — Illegal / Fictitious Dividend Distribution
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 438(1-2)',
    title:         'Illegal Distribution of Fictitious Dividends',
    severity:      critical,
    description:   'Dividends are distributed without net profits shown in an approved balance sheet, constituting illegal fictitious dividends.',
    recommendation: 'Distribute dividends strictly from verified net profits in approved balance sheets as mandated by Article 438(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(fictitious_dividend_distributed, true).

%% ---------------------------------------------------------------------------
%% Article 438(3) — Dividend Payment Period Limit (Max 4 Months)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 438(3)',
    title:         'Excessive Delay in Dividend Payment',
    severity:      warning,
    description:   Desc,
    recommendation: 'Disburse declared dividends within 4 months from the date of the general meeting resolution per Article 438(3).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(dividend_payment_delay_months, Months),
    number(Months),
    Months > 4,
    format(atom(Desc),
           'Dividend payment timeframe (~w months) exceeds the maximum statutory limit of 4 months from resolution date under Article 438(3).',
           [Months]).

%% ---------------------------------------------------------------------------
%% Article 495(4) — Private Limited Company (PLC) Member Count (2 to 50 Members)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 495(4)',
    title:         'Invalid PLC Member Count',
    severity:      critical,
    description:   Desc,
    recommendation: 'Maintain between 2 and 50 members for a standard Private Limited Company per Article 495(4).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(plc_members_count, Count),
    number(Count),
    ( Count < 2 ; Count > 50 ),
    format(atom(Desc),
           'PLC has ~w member(s), violating the statutory requirement of 2 to 50 members under Article 495(4).',
           [Count]).

%% ---------------------------------------------------------------------------
%% Article 496(1) — Private Limited Company Minimum Capital (Min 15,000 ETB)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 496(1)',
    title:         'PLC Capital Below Statutory Minimum',
    severity:      critical,
    description:   Desc,
    recommendation: 'Ensure PLC capital is at least 15,000 Ethiopian Birr as mandated by Article 496(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(company_capital, Capital),
    number(Capital),
    Capital < 15000,
    format(atom(Desc),
           'PLC capital (~w ETB) is below the statutory minimum of 15,000 ETB required by Article 496(1).',
           [Capital]).

check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 496(1)',
    title:         'PLC Capital Below Statutory Minimum',
    severity:      critical,
    description:   Desc,
    recommendation: 'Set PLC capital to at least 15,000 ETB per Article 496(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(plc_capital, Capital),
    number(Capital),
    Capital < 15000,
    format(atom(Desc),
           'PLC capital of ~w ETB is below the statutory minimum requirement of 15,000 ETB under Article 496(1).',
           [Capital]).

%% ---------------------------------------------------------------------------
%% Article 496(2) — PLC Minimum Share Par Value (Min 100 ETB)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 496(2)',
    title:         'PLC Share Par Value Below Statutory Minimum',
    severity:      critical,
    description:   Desc,
    recommendation: 'Ensure each PLC share has an equal par value of at least 100 ETB per Article 496(2).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(share_par_value, ParValue),
    number(ParValue),
    ParValue < 100,
    format(atom(Desc),
           'PLC share par value (~w ETB) is below the statutory minimum of 100 ETB per share under Article 496(2).',
           [ParValue]).

%% ---------------------------------------------------------------------------
%% Article 495(1) & 499(1) — Full 100% Paid-Up Capital Requirement for PLC
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Articles 495(1) & 499(1)',
    title:         'Incomplete Initial Capital Payment for PLC',
    severity:      critical,
    description:   Desc,
    recommendation: 'Ensure 100% of PLC capital is fully paid up in advance into a blocked bank account prior to registration per Articles 495(1) and 499(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(paid_up_capital_percent, Pct),
    number(Pct),
    Pct < 100,
    format(atom(Desc),
           'PLC capital is only ~w% paid up, violating the statutory requirement of 100% full advance payment under Articles 495(1) & 499(1).',
           [Pct]).

%% ---------------------------------------------------------------------------
%% Article 498 — Failure to Convert Single Member PLC within 6 Months
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 498(1-2)',
    title:         'Unconverted Single-Member PLC Beyond 6 Months',
    severity:      critical,
    description:   Desc,
    recommendation: 'Amend memorandum of association to convert to a one-person PLC or introduce a second member within 6 months per Article 498.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(single_member_unconverted_months, Months),
    number(Months),
    Months > 6,
    format(atom(Desc),
           'PLC membership has remained below two for ~w months without converting to a one-person PLC, breaching the 6-month limit under Article 498.',
           [Months]).

%% ---------------------------------------------------------------------------
%% Article 528 — PLC Legal Reserve Fund Transfer (Min 5% until 10% Capital)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 528',
    title:         'Insufficient PLC Legal Reserve Allocation',
    severity:      warning,
    description:   Desc,
    recommendation: 'Transfer at least 5% of net profits annually to the legal reserve until it reaches 10% of total PLC capital per Article 528.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(legal_reserve_transfer_percent, Rate),
    number(Rate),
    Rate < 5,
    format(atom(Desc),
           'PLC legal reserve deduction (~w%) is below the statutory minimum of 5% of net profits required under Article 528.',
           [Rate]).

%% ---------------------------------------------------------------------------
%% Article 66 — Preservation of Accounting Books & Records (Min 10 Years)
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 66',
    title:         'Insufficient Accounting Records Retention Period',
    severity:      warning,
    description:   Desc,
    recommendation: 'Maintain all accounting books and financial records for at least 10 years per Article 66.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(accounting_records_retention_years, Years),
    number(Years),
    Years < 10,
    format(atom(Desc),
           'Accounting records retention period (~w years) is below the statutory requirement of 10 years mandated by Article 66.',
           [Years]).

%% ---------------------------------------------------------------------------
%% Articles 180 & 495(5) — Prohibited Transferable Securities Issuance by PLC
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Articles 180 & 495(5)',
    title:         'Unlawful Issuance of Transferable Securities by PLC',
    severity:      critical,
    description:   'PLC or business organization other than a Share Company has issued or proposed transferable securities/bonds.',
    recommendation: 'Prohibit public subscription and transferable securities issuance; only Share Companies may issue transferable securities under Articles 180 & 495(5).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(issues_transferable_securities, true).

%% ---------------------------------------------------------------------------
%% Article 257 — Missing Independent Expert Valuation for Non-Cash Contributions
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 257',
    title:         'Missing Independent Valuation for In-Kind Contributions',
    severity:      critical,
    description:   'Non-cash contributions (in-kind assets) were contributed without an independent expert valuation report.',
    recommendation: 'Obtain a detailed independent expert valuation report for all in-kind property contributions per Article 257.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(has_contribution_in_kind, true),
    \+ doc_fact(has_expert_valuation_report, true).

%% ---------------------------------------------------------------------------
%% Article 261 — Missing External Audit Verification of Company Formation
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 261',
    title:         'Missing External Audit of Formation Procedure',
    severity:      warning,
    description:   'Share company formation procedure lacks verification by independent external auditors.',
    recommendation: 'Ensure company formation compliance is audited by external auditors before subscriber meetings per Article 261.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, share_company),
    \+ doc_fact(has_external_formation_audit, true).

%% ---------------------------------------------------------------------------
%% Article 402(2) — Unanimous Vote Requirement for Nationality Change
%% ---------------------------------------------------------------------------
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 402(2)',
    title:         'Non-Unanimous Vote for Company Nationality Change',
    severity:      critical,
    description:   'Company resolution to change nationality was passed without unanimous consent of all shareholders.',
    recommendation: 'Require unanimous vote of all share owners to approve changing the company nationality per Article 402(2).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(nationality_change_vote_unanimous, false).


%% ============================================================================
%% Three-Valued Logic: Ambiguity Warning Rules (Phase 4)
%%
%% Triggered when a clause TYPE was detected by NER (has_*_clause = true)
%% but the numeric value could not be extracted from the document text.
%% These fire `severity: warning` to flag clauses requiring manual review.
%% ============================================================================

%% Article 247(1) — Capital clause detected but amount not parseable
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 247(1)',
    title:         'Ambiguous Company Capital Amount',
    severity:      warning,
    description:   'Contract references Share Company capital but the exact ETB figure could not be extracted. Manual review required to confirm the 50,000 ETB minimum.',
    recommendation: 'State exact registered capital in ETB (minimum 50,000 ETB for Share Company) per Article 247(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, share_company),
    doc_fact(has_capital_clause, true),
    \+ doc_fact(company_capital, _),
    \+ doc_fact(share_company_capital, _).

%% Article 496(1) — PLC Capital clause detected but amount not parseable
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 496(1)',
    title:         'Ambiguous PLC Capital Amount',
    severity:      warning,
    description:   'Contract references PLC capital but the exact ETB figure could not be extracted. Manual review required to confirm the 15,000 ETB minimum.',
    recommendation: 'State exact registered PLC capital in ETB (minimum 15,000 ETB) per Article 496(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(company_type, plc),
    doc_fact(has_capital_clause, true),
    \+ doc_fact(company_capital, _),
    \+ doc_fact(plc_capital, _).

%% Article 296(1) — Board composition clause detected but member count not parseable
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 296(1)',
    title:         'Ambiguous Board of Directors Size',
    severity:      warning,
    description:   'Articles of association reference a Board of Directors but the total number of members could not be determined. Manual review required to verify compliance with 3 to 13 directors.',
    recommendation: 'Specify exact board size (3 to 13 directors) per Article 296(1).'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(has_board_clause, true),
    \+ doc_fact(board_directors_count, _).

%% Article 434 / 528 — Reserve clause detected but percentage not parseable
check_violation(Domain, violation{
    rule:          'Ethiopian Commercial Code Proclamation No. 1243/2021 - Article 434 & 528',
    title:         'Ambiguous Legal Reserve Allocation Percentage',
    severity:      warning,
    description:   'Document references a legal reserve fund but the exact annual net profit deduction rate could not be extracted. Manual review required.',
    recommendation: 'Specify annual transfer of at least 5% of net profits to the legal reserve fund per Articles 434 & 528.'
}) :-
    ethiopian_commercial_domain(Domain),
    doc_fact(has_reserve_clause, true),
    \+ doc_fact(legal_reserve_transfer_percent, _),
    \+ doc_fact(plc_legal_reserve_transfer_percent, _).

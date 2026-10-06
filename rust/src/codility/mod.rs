mod binary_gap_flip;
pub mod in_memory_dynamodb;
mod inherited_access;
mod lru_cache;
mod message_normalizer_router;
mod per_tenant_rate_limiter;
mod per_tenant_rate_limiter_colidity;
mod permission_inheritance_on_graph;
mod temper_evident_audit_log;
mod ttl_lru_cache;

/*
Implement 4 & 5, as others are related...!!!!
most relevant:
#4
in_memory_dynamo_db
ttl_lru_cache
message_normalizer_router
temper_evident_audit_log
#2
#1
#5
#3


1. Message deduplication — 25 minutes
Given messages (tenant_id, message_id, timestamp), return only the first occurrence of each message, preserving input order.
- Message IDs are unique within a tenant, not globally.
- Input can contain up to 200,000 messages.
- Target: O(n) expected time.
- Follow-up: Deduplicate only if the same message was accepted within the previous 60 seconds; assume timestamps are nondecreasing.
Tests: Empty input, repeated duplicates, same message ID across tenants, time-window boundaries.

2. Per-tenant rate limiter — 35 minutes
Given requests (tenant_id, timestamp_ms) in nondecreasing timestamp order, return whether each request is accepted.
Each tenant may have at most K accepted requests in the rolling window (t − W, t]. Rejected requests do not count.
- Up to 200,000 requests.
- Target: O(n) total time using hash maps and queues.
- Follow-up: How would concurrent Lambda instances change the implementation?
Tests: Identical timestamps, exact expiry boundary, interleaved tenants, long idle gaps.

3. Highest contiguous message sequence — 30 minutes
Messages arrive as (tenant_id, sequence_number), possibly duplicated or out of order. Sequences start at 1.
After every arrival, return that tenant’s highest sequence number for which all preceding messages have arrived.
Example for one tenant:
Arrivals: 2, 3, 1, 5, 3, 4
Outputs:  0, 0, 3, 3, 3, 5

- Target: O(n) expected total time.
- Follow-up: How would you bound memory if a sequence gap never closes?
Tests: Duplicate old messages, large gaps, independent tenant progress.

4. Nested-group access checking — 40 minutes
You receive:
- Group membership edges: a user or group can belong to another group.
- Document grants: documents grant access to users or groups.
- A query: can user U read document D?
Implement access checking through transitive membership.
- Every entity belongs to a tenant.
- Cross-tenant edges and grants must be ignored.
- Membership may contain cycles.
- Target: O(V + E) per query.
- Follow-up: Cache repeated checks while handling membership revocation.
Tests: Direct access, nested groups, cycles, disconnected groups, cross-tenant links.

5. Merge message histories — 40 minutes
Given k channel histories, each sorted newest-first, return the newest N unique messages across all channels.
Each message contains (timestamp, message_id). All histories belong to one tenant; duplicates have identical timestamps. Break timestamp ties by message ID ascending.
- Avoid flattening and sorting the entire input.
- Target: O(k + p log k), where p is the number of entries examined.
- Follow-up: Design a cursor to fetch the next page.
Tests: Empty channels, duplicates across channels, tied timestamps, fewer than N unique messages.
Suggested first mock: Exercises 1 + 2 + 3 in 90 minutes, including tests and a brief complexity explanation.


Even better ones:
Search index with tenant and permission filtering (would rank 2). Build an inverted index over messages supporting AND and phrase queries, returning only hits from the caller's tenant that the caller is allowed to read. This matches OpenSearch integration plus filtering results by permission, which is where search leaks happen.

Signed token validation (would rank 3). Parse and verify an HMAC-signed JWT-style token: check the signature, expiry and clock-skew, tenant_id and scope claims, and return a typed error for each failure. This covers the spec's authentication requirement, which none of the current exercises do.

Tenant-scoped repository (would rank 4). Wrap a store so no query can run without a TenantId, enforced by the type system (a newtype, and no unscoped methods). Add an explicit, audited path for cross-tenant analytics. It tests "tenant isolation at the data and API layer" and shows good Rust design.


Message delivery with retry (would rank 6). Deliver each message to a channel with exponential backoff and jitter, add a per-channel circuit breaker, and move a message to a dead-letter queue after N failures. It covers message routing with the delivery failures you get in production.

*/

/*

Corro invites you to complete a technical assessment

access_time
80
minutes for
draft
2
tasks
Before you begin
There are 2 tasks in the test. You can solve them in any order.
There's no option to pause. Make sure you will not be interrupted for 80 minutes.
Your solution(s) should consider all possible corner cases and handle large input efficiently. Passing the example test does not indicate that your solution is correct. The example test is not part of your final score.
After finishing the test you will receive feedback containing your score. See example feedback.
If you accidentally close your browser, use the invitation link to get back to your test.
You can write your solution(s) in C, C#, C++, Dart, Go, Java 11, Java 21, JavaScript, Kotlin, Lua, Objective-C, PHP, Pascal, Perl, Python, Ruby, Rust, Scala, Swift, TypeScript or Visual Basic.

FAQ arrow_right_alt
learn more about Codility tests

Feedback example arrow_right_alt
see an example of what you will receive after finishing the test

What behaviors will be tracked during the assessment?
keyboard
mouse
videocam
To protect the integrity of today’s assessment and ensure compliance with hiring policies, the hiring team has enabled features that track and record in-app behavior during the assessment.The actions you take in the Codility environment will be tracked, recorded, and made available to the hiring team. No decisions are made automatically from these recordings.
Do you need help?
If you require special arrangements to be made to take the test, please inform your test sponsor and they will try to provide support.

In case of technical problems with your test, please contact support@codility.com.
Enable accessibility mode for screen readers
Are you ready?
I have read and accepted Codility's Terms of Service  and  Privacy Policy.

I will follow Codility’s Code of Honour and I understand that my solution will be checked for plagiarism.


*/

def numDuplicates(name, price, weight):
    """
    count number of duplicates
    """
    return len(name) - len(set(zip(name, price, weight)))


def droppedRequests__solution2(requestTime):
    # fetched from the internet
    dropped = 0
    # this is to keep track of any of the element that is already dropped due to any of 3 limit violation.
    already_dropped = {}
    for i in range(len(requestTime)):
        if i > 2 and requestTime[i] == requestTime[i-3]:
            if requestTime[i] not in already_dropped or already_dropped[requestTime[i]] != i:
                already_dropped[requestTime[i]] = i
                dropped += 1
                print('1s rule:', i, requestTime[i])

        elif i > 19 and requestTime[i] - requestTime[i-20] < 10:
            if requestTime[i] not in already_dropped or already_dropped[requestTime[i]] != i:
                already_dropped[requestTime[i]] = i
                dropped += 1
                print('20s rule:', i, requestTime[i])

        elif i > 59 and requestTime[i] - requestTime[i-60] < 60:
            if requestTime[i] not in already_dropped or already_dropped[requestTime[i]] != i:
                already_dropped[requestTime[i]] = i
                dropped += 1
                print('60s rule:', i, requestTime[i])

    return dropped


def droppedRequests__my_solution(requestTime):
    """
    count dropped requests as per rules:
    - no more than 3 reqs/sec
    - no more than 20 reqs/10sec
    - no more than 60 reqs/60sec
    ...NOTE: dropped requests count towards above quotas
    https://leetcode.com/discuss/interview-question/819577/Throttling-Gateway-Hackerrank
    """
    import collections
    sec_counts = collections.OrderedDict({i: requestTime.count(i) for i in requestTime})
    all_dropped = 0
    for sec in range(0, max(sec_counts.keys())+1):
        if sec in sec_counts:
            # deal with 1 sec
            count_1s = sec_counts[sec]
            delta_1s = count_1s - 3
            if delta_1s > 0:
                all_dropped += delta_1s

            # deal with 10 secs
            all_in_10s = count_1s - max(delta_1s, 0)
            for j in range(max(sec-9, 0), sec):
                if j in sec_counts:
                    all_in_10s += sec_counts[j]
            delta_10s = all_in_10s - 20
            if delta_10s > 0:
                all_dropped += delta_10s

            # deal with 60 secs
            all_in_60s = count_1s - max(delta_10s, 0)
            for j in range(max(sec-59, 0), sec):
                if j in sec_counts:
                    all_in_60s += sec_counts[j]
            delta_60s = all_in_60s - 60
            if delta_60s > 0:
                all_dropped += delta_60s
    return all_dropped


if __name__ == "__main__":
    requestTime = [1,1,1,1,2,2,2,3,3,3,4,4,4,5,5,5,6,6,6,7,7,7,7,11,11,11,11]
    print('droppedRequests__solution2:', droppedRequests__solution2(requestTime))
    print('\n')
    print('droppedRequests__my_solution:', droppedRequests__my_solution(requestTime))

    print('\n')
    n = numDuplicates(
        ['ball', 'bat', 'glove', 'glove', 'glove'],
        [2, 3, 1, 2, 1],
        [2, 5, 1, 1, 1])
    print('numDuplicates:', n)

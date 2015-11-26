"""
print count of all non-whitespace chars in a string, sorted
"""

from collections import defaultdict
import re


def count_chars(s):
    d = defaultdict(int)
    for c in re.sub(r'\s', r'', s):
        d[c] += 1
    for c in sorted(d):
        print c, d[c]


count_chars("this is 1 string of text")


############################

"""
print average of stopwatch times eg. "00:02:20" and "00:04:40" -> "00:03:30",
where format is "MM:SS:HS" where MM = minutes; SS = seconds; and HS = hundredths of a second. 
"""


def t_avg(t1, t2):
    def t_secs(t):
        mm, ss, hs = [int(x) for x in t.split(':')]
        return mm * 6000 + ss * 100 + hs
    avg = (t_secs(t1) + t_secs(t2))/2
    mm, hs = divmod(avg, 6000)
    ss, hs = divmod(hs, 100)
    return '%02d:%02d:%02d' % (mm, ss, hs)


print t_avg("00:02:20", "00:04:40")
print t_avg("01:02:20", "00:04:40")
print t_avg("03:03:30", "01:01:10")

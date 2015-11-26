"""
A performant representation of Date/Calendar class to look up is_holiday() method.
"""

from array import array
from datetime import datetime, timedelta


class Date(object):
    def __init__(self, date_time):
        self.day_from_epoch = int(date_time.strftime('%s'))/(24*60*60)

    def next_day(self):
        self.day_from_epoch += 1;


class Calendar(object):
    """
    holiday represents weekends only, 1/1/1970 was a Thursday.  Roughly 100 years from 1970, ie. till 2070
    """
    holiday = array('b', [False, False, True, True, False, False, False]*52*100)

    def is_holiday(self, date):
        return bool(self.holiday[date.day_from_epoch])


if __name__ == '__main__':
    d = Date(datetime.now())
    c = Calendar()
    for i in xrange(7):
        print 'day from today %d, is holiday - %r' % (i, c.is_holiday(d))
        d.next_day()




"""
1000 of bottles of wine, 1 poisoned, infinite number of lab rats. Feed any rat any number of sips to check which bottle is poisoned - with minimal number of rats.

Ans: 1000 bottles can be represented by 10^10=1024, ie:
0000000000
0000000001
0000000010
0000000011
0000000100
0000000101

Feed bottle no. 5 to rats 1 and 3 (101), if only those 2 are dead in the morning - 5's the culprit.
"""
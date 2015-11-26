from unittest import TestCase
from exercise import exercise1, get_members
import data1
import data2
import data3
import data4


class TestExercises(TestCase):
    def test_exercise1_1(self):
        teams = sorted([t.displayname for t in exercise1(data1.alice, data1.people)])
        self.assertEquals(['The A-Team', 'The C-Team'], teams)

    def test_exercise1_2(self):
        teams = sorted([t.displayname for t in exercise1(data2.alice, data2.people)])
        self.assertEquals(['The A-Team', 'The C-Team'], teams)

    def test_exercise1_3(self):
        teams = sorted([t.displayname for t in exercise1(data3.alice, data3.people)])
        self.assertEquals(['The A-Team', 'The B-Team', 'The C-Team'], teams)

    def test_exercise1_4(self):
        teams = sorted([t.displayname for t in exercise1(data4.alice, data4.people)])
        self.assertEquals(['The A-Team', 'The B-Team', 'The C-Team'], teams)

    def test_get_members_2(self):
        members = sorted(p.displayname for p in get_members(data2.c_team))
        self.assertEquals(['Alice', 'Bob', 'Carlos', 'Charlie', 'Eve'], members)

    def test_get_members_3(self):
        members = sorted(p.displayname for p in get_members(data3.c_team))
        self.assertEquals(['Alice', 'Bob', 'Carlos', 'Charlie', 'Eve', 'Peggy', 'Trent', 'Victor'], members)

    def test_get_members_4(self):
        members = sorted(p.displayname for p in get_members(data4.c_team))
        self.assertEquals(['Alice', 'Bob', 'Carlos', 'Charlie', 'Eve', 'Peggy', 'Trent', 'Victor'], members)

from person import Person

alice = Person(0, 'Alice')
bob = Person(1, 'Bob')
carlos = Person(2, 'Carlos')
carol = Person(3, 'Carol')
charlie = Person(4, 'Charlie')
chuck = Person(5, 'Chuck')
dave = Person(6, 'Dave')
eve = Person(7, 'Eve')
mallory = Person(8, 'Mallory')
peggy = Person(9, 'Peggy')
trent = Person(10, 'Trent')
victor = Person(11, 'Victor')
walter = Person(12, 'Walter')

a_team = Person(90, 'The A-Team', [alice, bob, carlos])
b_team = Person(91, 'The B-Team', [peggy, trent, victor, bob])
c_team = Person(92, 'The C-Team', [charlie, eve, alice, bob])

people = [alice, bob, carlos, carol, charlie, chuck, dave, eve, mallory,
    peggy, trent, victor, walter, a_team, b_team, c_team]

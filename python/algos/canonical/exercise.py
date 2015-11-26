def exercise1(person, people):
    """Returns teams the person belongs to."""
    cache = {}

    def contains_person(team):
        """Returns True if person is in team or it's descendants, caches this info in `cache`.

        `cache` can be used to generate whitelist of teams with the `person`.
        """

        if team not in cache:
            people = (m for m in team.members if not m.is_team)
            if person in people:
                cache[team] = True
            else:
                teams = (m for m in team.members if m.is_team)
                cache[team] = any(contains_person(t) for t in teams)
        return cache[team]

    teams = (p for p in people if p.is_team)
    for t in teams:
        contains_person(t)
    whitelist = [t for t, whitelisted in cache.iteritems() if whitelisted]
    return whitelist


def get_members(team):
    cache = {}

    def collect_descendants(team):
        """Collects descendants in `cache`.

        `cache` can be used to aggregate all members of the team and it's descendant.
        """

        if team not in cache:
            people = [m for m in team.members if not m.is_team]
            cache[team] = people
            teams = (m for m in team.members if m.is_team)
            for m in teams:
                collect_descendants(m)

    collect_descendants(team)
    all_descendants = sum(cache.values(), [])
    return set(all_descendants)

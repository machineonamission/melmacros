macros have 3 components they can be searched by

- name/alias (macro/aliases are unique within group. macro has one name and 0+ aliases)
- macro group (owned by user/server, group names are unique within owner)
- owner (user/server)

macros can be named differently depending on context:

- `name` if name alone distinguishes, thats fine
- `group/name` if duplicate names
- `owner/group/name` if duplicate groups
- `owner (owner ID/username)/group/name` if duplicate owner names somehow

macros will change their representation dynamically in context, only distinguishing further if necessary

user can search by these representations. the number of slashes in their query changes what the search means 

eg `word` searches for macros named/aliased `word`, but `group/word` searches for macros named/aliased `word` in group `group`

macro search is scoped by if user is subscribed to the macro group, this is trivial to handle just some inner joins

so names, aliases, groups, and owners need to be full text searchable, definitely including partial matches.

sqlite has FTS5 but it's bizarre when trying to mix it with non text data

i dont know if its more efficient to do this after the query is returned in rust, or in sql directly (would be nice to sql limit search results)


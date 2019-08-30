Con RS
======

Command line contact management tool.

## Contacts File

Contacts are stored into an UTF-8 encoded JSON file with the following
structure:

 * `contacts` (list) -- a list of [contact](#contact) objects.

This file is stored in `$XDG_DATA_HOME/con-rs/contacts.json` or
`$HOME/.local/share/con-rs/contacts.json` if `$XDG_DATA_HOME` is not specified.

## Contact

Contact is a (JSON) object with the following structure/properties. All string
keys and properties are Unicode.

Some contact properties are maps, where keys serve as individual item
identifiers / names and values as the items. For example there is a map `email`
which maps e-mail names (e.g. `personal`) to e-mail addresses (e.g.
`john@example.com`). Each key in a map must be unique.

 * `fullName` (str) -- full name of the contact. This should be set only for
   persons.
 * `entityName` (str) -- name of the non-person entity.
 * `tels` (map) -- a map of telephone numbers.
 * `emails` (map) -- a map of e-mail addresses.
 * `labels` (map) -- a map of arbitrary key value labels which can be used for
   various purposes including unique contact identification (e.g. `id` ->
   `<unique ID HERE>`) or contact searching and filtering.

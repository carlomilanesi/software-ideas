# Glossary

## The problems

Every application domain has its own terminology, which is often abbreviated in acronyms.

And very often, single customers or single software shops have their own terminology, which is often abbreviated in acronyms, too.

This raises the following issues:
1. **Newcomers**. Newcomers understand little and feel disoriented, because they do not know such terminology.
2. **Ambiguity**. A single word or phrase can have several meanings. For example, in computer technology, "directory" can mean a file system cataloging structure that contains references to computer files or to other directories, but it can also mean a name service which maps the names of network resources to their respective network addresses. Another example is using the name "database" both for a set of data and for the software used to manage it, whose proper name is "database manager" or "database management system". Of course, this can lead to ambiguity.
3. **Synonyms**. A single concept can be named by several words or phrases. For example, a file system directory can be also named "folder".
4. **Extensive definitions**. A concept is named by a sequence of sub-concepts. For example, a database can have a table "people_and_companies", to specify counterparties to contracts (such as sales), or a table can be "regions_nations_and_states" to specify geographical areas. As another example, to refer to binary software the phrase "binary programs or binary libraries" is used.

## The solutions

Here are the solutions to such issues:
1. **Newcomers**. A glossary should be created and maintained by a software architect. Such glossary should contains jargon terms, acronyms, but also common words or phrases which have a specific meaning for the organization. Anyone, but particularly newcomers, can benefit from consulting such glossary whenever they are not sure about the meaning of a word, a phrase, or an acronym. It would be very useful to use a documentation viewer which shows a tooltip for every word described in such glossary.
2. **Ambiguity**. A company policy should state that, in technical documents, any word and any phrase should have only one meaning. For example, if "directory" can be ambiguous in the organization setting, the word "folder" should be used everywhere in its place. Such definitions should be included in the glossary.
3. **Synonyms**. A company policy should state that, in technical documents, for every concept, a single word or phrase should be used. For example, because if the word "database" is used to mean a set of data, the acronym "DBMS" should be used to mean the software used to handle databases. Alternatively, the word "database" can be used to means the software, and "database instance" should be used everywhere to mean a set of data. Such definitions should be included in the glossary.
4. **Extensive definitions**. A company policy should state that, in technical documents, whenever a sequence of sub-concepts is used repeatedly, it should be replaced by a single word or phrase. For examples, instead of "people_and_companies", "counterparties" should be used; instead of "regions_nations_and_states", "areas" should be used; instead of "binary programs or binary libraries", "binary modules" or "binaries" should be used. Such definitions should be included in the glossary.

Files called AGENTS.md commonly appear in many places inside a container - at "/", in "~", deep within git repositories, or in any other directory; their location is not limited to version-controlled folders. Globim also treats files called GLOBIM.md as supplemental project instructions.

Their purpose is to pass along human guidance to you, the agent. Such guidance can include coding standards, explanations of the project layout, steps for building or testing, and even wording that must accompany a GitHub pull-request description produced by the agent; all of it is to be followed.

Each AGENTS.md or GLOBIM.md governs the entire directory that contains it and every child directory beneath that point. Whenever you change a file, you have to comply with every AGENTS.md and GLOBIM.md whose scope covers that file. Naming conventions, stylistic rules and similar directives are restricted to the code that falls inside that scope unless the document explicitly states otherwise.

When two project instruction files disagree, the one located deeper in the directory structure overrides the higher-level file. When AGENTS.md and GLOBIM.md both exist in the same directory, GLOBIM.md is supplemental and may refine instructions for Globim-specific work. Instructions given directly in the prompt by the system, developer, or user outrank any AGENTS.md or GLOBIM.md content.

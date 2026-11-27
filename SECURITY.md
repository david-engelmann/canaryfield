# Security Policy

## Supported Versions

Security fixes land on the latest `0.46.x` and subsequent `0.x` release line on `main`. Earlier tags do not receive backported patches.

## Security Model

Canaryfield copies the `--target` directory into temporary scratch folders (or throwaway local git clones under `--plant git-branches`), plants synthetic AST defects into those scratch copies, and executes each `--gate NAME=COMMAND` string through `sh -c` under the current user account while leaving the original target directory untouched. **Running Canaryfield executes your supplied gate commands and the target repository's test or build code as the invoking OS user.** Only run gates over untrusted repositories inside an isolated container or VM, and never commit real paths, credentials, or private source trees into fixtures.

## Reporting a Vulnerability

Do not open a public issue for a suspected security vulnerability. Report vulnerabilities privately through [GitHub Security Advisories](https://github.com/david-engelmann/canaryfield/security/advisories/new) using synthetic inputs only, naming the affected version and the relevant section of `CONTRACT.md`. Every report receives an initial review within seven days, and confirmed fixes ship with a regression pin in the test suite before public disclosure.

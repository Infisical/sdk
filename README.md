<h1 align="center">
  <img width="300" src="/resources/logo.svg#gh-dark-mode-only" alt="infisical">
</h1>
<p align="center">
  <p align="center"><b>Infisical cross-language SDK </b></p>
<h4 align="center">
  <a href="https://infisical.com/slack">Slack</a> |
  <a href="https://infisical.com/">Infisical Cloud</a> |
  <a href="https://infisical.com/docs/self-hosting/overview">Self-Hosting</a> |
  <a href="https://infisical.com/docs/documentation/getting-started/introduction">Docs</a> |
  <a href="https://www.infisical.com">Website</a>
</h4>

<h4 align="center">
  <a href="https://github.com/Infisical/infisical/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-blue.svg" alt="Infisical is released under the MIT license." />
  </a>
  <a href="https://github.com/infisical/infisical/blob/main/CONTRIBUTING.md">
    <img src="https://img.shields.io/badge/PRs-Welcome-brightgreen" alt="PRs welcome!" />
  </a>
  <a href="https://github.com/Infisical/infisical/issues">
    <img src="https://img.shields.io/github/commit-activity/m/infisical/infisical" alt="git commit activity" />
  </a>
  <a href="https://infisical.com/slack">
    <img src="https://img.shields.io/badge/chat-on%20Slack-blueviolet" alt="Slack community channel" />
  </a>
  <a href="https://twitter.com/infisical">
    <img src="https://img.shields.io/twitter/follow/infisical?label=Follow" alt="Infisical Twitter" />
  </a>
</h4>

## Introduction

**[Infisical](https://infisical.com)** is the open source secret management platform that teams use to centralize their secrets like API keys, database credentials, and configurations.

The motive behind creating a cross-language SDK was the overhead that comes with maintaining a large collection of SDK's. By having an easily portable SDK written in one singular language, we greatly cut down on the amount of duplicate code across the different SDK's

Sometimes you may notice that an SDK in one language is missing features that an SDK for another language has. By writing all the SDK logic in a single language, that issue will no longer be relevant, as porting functionality should take less than a minute.

We chose Rust as the foundation for our SDK based on several key factors, primarily its exceptional performance and versatility. Rust's extensive libraries simplify the process of building for multiple languages, with somewhat easy cross-architecture compilation. This robust ecosystem streamlines the creation of cross-language SDKs, optimizing the entire development process. Our research indicated that Rust is one of the most battle-tested languages for this purpose.

## Current SDK's

-   **[Node.js SDK](https://github.com/Infisical/sdk/tree/main/languages/node) — [(NPM)](https://www.npmjs.com/package/@infisical/sdk)**
-   **[Python SDK](https://github.com/Infisical/sdk/tree/main/crates/infisical-py) — [(PyPI)](https://pypi.org/project/infisical-python/)**
-   **[Java SDK](https://github.com/Infisical/sdk/tree/main/languages/java) — [(GitHub Maven Registry)](https://github.com/Infisical/sdk/packages/2019741)**
-   **[C# SDK](https://github.com/Infisical/sdk/tree/main/languages/csharp)  - [(NuGet)](https://www.nuget.org/packages/Infisical.Sdk)**
-   Many more to come!

## Security

Please do not file GitHub issues or post on our public forum for security vulnerabilities, as they are public!

Infisical takes security issues very seriously. If you have any concerns about Infisical or believe you have uncovered a vulnerability, please get in touch via the e-mail address security@infisical.com. In the message, try to provide a description of the issue and ideally a way of reproducing it. The security team will get back to you as soon as possible.

Note that this security address should be used only for undisclosed vulnerabilities. Please report any security problems to us before disclosing it publicly.


## Contributing

Whether it's big or small, we love contributions. Check out our guide to see how to [get started](https://infisical.com/docs/contributing/getting-started).

Not sure where to get started? Join our <a href="https://infisical.com/slack">Slack</a>, and ask us any questions there.

# ai prompting techniques

## Overview

Effective prompting is the foundation of AI-first crypto development. This guide covers proven approaches and recommended patterns that help generate production-quality code and handle complex blockchain scenarios efficiently.

## Core approaches

### Context layering

Build context progressively from general to specific:

```
"I'm building a consumer crypto wallet using CDP embedded wallets and Next.js.
The app uses CDP Web SDK and OnchainKit components. Here's my current structure: [paste code]
Add a send money feature with email notifications for recipients without wallets."

```

### Error-driven development

Use errors as learning opportunities:

```
"I'm getting this error: [exact error message]
Code: [paste code]
Goal: [what I was trying to achieve]
Fix this and explain why it happened so I can avoid similar issues."

```

**Key elements:**

-   Exact error messages (don’t paraphrase)
-   Complete code context
-   Clear intent
-   Ask for explanations

### Comparative analysis

Get AI to compare approaches:

```
"I need to implement [functionality]. I'm considering:
Approach A: [description]
Approach B: [description]
Which would you recommend for [app type] and why? Consider security, UX, and complexity."

```

## Crypto-specific patterns

### Wallet integration

```
"Integrate [wallet type] with my [app description].
Requirements:
- [user flows]
- [security needs]
- [network support]
Handle: connection states, network switching, error recovery, mobile compatibility."

```

### Transaction flows

```
"Design a transaction flow for [action].
User journey: [steps]
Constraints: [gas limits, network]
Security: [validation requirements]
Include user feedback, error handling, and status tracking."

```

## Recommended patterns

### Build in stages

Start simple, then improve: **First, get it working:**

```
"Create basic [feature] for CDP app. Focus on core functionality."

```

**Then, make it better:**

```
"Improve this implementation: [paste code]
Add error handling, loading states, validation."

```

**Finally, polish it up:**

```
"Optimize for production: [paste code]
Focus on performance, security, testing."

```

### Ask for different perspectives

Get focused feedback by asking the AI to wear different “hats”: **For architecture decisions:** “Focus only on high-level design decisions and system integration.” **For implementation:** “Write clean, performant code. Assume architecture is decided.” **For security review:** “Review only for vulnerabilities and compliance issues.”

## Best practices

### Do

-   Be specific with requirements and context
-   Include exact error messages and code
-   Ask for explanations, not just solutions
-   Build complexity incrementally

### Don’t

-   Use vague requests like “make it better”
-   Skip project context
-   Accept code without review
-   Rush through complex features

## What to read next

Apply these techniques in:

-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Core development patterns and workflow processes
-   **[Debugging AI Code](https://developer.chrome.com/get-started/develop-with-ai/development/ai-debugging)**: Systematic debugging approaches for AI-generated code
-   **[Testing Strategies](https://developer.chrome.com/get-started/develop-with-ai/development/ai-testing)**: AI-assisted testing methodologies and best practices
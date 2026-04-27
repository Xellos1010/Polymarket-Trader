# ai debugging

## Overview

When you’re building crypto apps with AI help, things often go wrong with wallets, network connections, and transactions. This guide shows you a simple, step-by-step way to debug these issues. **Here’s how to debug effectively using AI:**

-   **Isolate**: Create a simple example that shows the exact problem
-   **Analyze**: Give the AI context about your app and what you were trying to do
-   **Fix**: Ask for a solution that’s secure and handles edge cases
-   **Prevent**: Get tests written so this bug doesn’t happen again

## Debug prompt template

**Here’s a template you can copy and fill in:**

```
"Debug this issue in my [app type]:
Error: [exact message]
Expected vs Actual: [one line]
Environment: [browser/network/wallet]
Minimal code:
[paste tiny repro]
Please provide:
1) Likely root cause(s)
2) Corrected code
3) Safer pattern to avoid this
4) Tests to prevent regression (framework: [Jest/Vitest])"

```

## Process

## How to prevent bugs

### Always review AI code

**Use this prompt to get the AI to check its own work:**

```
"Review your generated code for:
- Common crypto development mistakes
- Security vulnerabilities  
- Performance issues
- Integration problems
Provide critical analysis and improvements."

```

### Generate tests with code

**Ask for tests when you request features:**

```
"Generate [feature] with comprehensive tests:
Requirements: [feature needs]
Context: [existing code]
Provide implementation plus unit/integration/error tests."

```

### Ask for explanations

**Don’t just ask for code, ask the AI to explain its decisions:**

```
"Implement [feature] with detailed explanations:
Requirements: [needs]
Context: [existing code]
Explain each major decision and potential issues."

```

## Common crypto app errors

### Transaction errors

**When transactions fail, try this:**

```
Pattern: "execution reverted" or "transaction failed"
Causes: Insufficient gas, invalid parameters, network congestion
Debug: "Transaction reverting: [error]. Details: [transaction object]. Help identify why."

```

### Connection errors

**When wallets won’t connect, try this:**

```
Pattern: "wallet not connected" or "provider not found"  
Causes: Missing extension, wrong network, state management issues
Debug: "Users can't connect: [error]. Code: [connection logic]. What to check?"

```

### State errors

**When your app’s state gets messed up, try this:**

```
Pattern: "Cannot read property of undefined" or infinite re-renders
Causes: Missing dependencies, race conditions, memory leaks
Debug: "State management issues: [symptoms]. Code: [component]. Fix patterns?"

```

## More debugging tips

### Compare working vs broken code

**Try this when you have a working example:**

```
"My code isn't working: [paste broken code]
Working example: [paste working code or link]
What's different and why might mine fail? How should I fix it?"

```

### Break down complex problems

**When the issue is complicated, try debugging piece by piece:**

```
"Complex issue with [feature]:
Part 1 - Basic functionality: [code] - Does this work?
Part 2 - Integration: [describe] - How should this integrate?
Part 3 - Edge cases: [describe] - What cases to handle?
Debug each part systematically."

```

### When something breaks later

**If your feature worked before but now it’s broken:**

```
"Feature worked before but now broken:
Last working: [commit/description]
Current error: [error message]
Recent changes: [list changes]
What change likely caused this regression?"

```

## What to read next

Apply these debugging techniques with:

-   **[AI Prompting Techniques](https://developer.chrome.com/get-started/develop-with-ai/development/ai-prompting-techniques)**: Better prompts lead to fewer bugs and more accurate AI responses
-   **[Testing Strategies](https://developer.chrome.com/get-started/develop-with-ai/development/ai-testing)**: Comprehensive testing prevents issues and catches bugs early
-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Systematic development processes reduce debugging needs
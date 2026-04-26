# ai troubleshooting

## Overview

This guide covers the most common issues you’ll encounter when building crypto applications with AI assistance, along with proven solutions and debugging strategies.

## Build and Deployment Issues

### Linting errors

Linting and Code Style Errors

**Problem:** Build fails with linting or code style errors after AI generates code**General debugging approach:**

1.  **Check your package.json** - Look for linting scripts (could be `lint`, `eslint`, `prettier`, etc.)
2.  **Run the linting command** - Each starter app may have different commands
3.  **Fix one error at a time** - Start with the first error in the output
4.  **Look for auto-fix options** - Many linters have `--fix` flags

**Debugging with AI:**

```
"My build is failing with this linting error: [paste exact error]
I'm using [starter app type: Consumer/DeFi/Agent]
Here's my package.json scripts: [paste scripts section]
How do I fix this?"

```

**Useful resources:**

-   Check your starter app’s README for linting setup
-   Look at your starter app’s existing code style patterns
-   Different starter apps may use ESLint, Prettier, or other tools

TypeScript Type Errors

**Problem:** AI-generated code has TypeScript errors**General debugging approach:**

1.  **Read the error message carefully** - TypeScript errors are usually specific
2.  **Fix one error at a time** - Start from the top of the error list
3.  **Check imports** - Make sure you’re importing types from the right packages
4.  **Look at existing code** - See how your starter app handles similar patterns

**Debugging with AI:**

```
"I'm getting this TypeScript error: [paste exact error]
I'm using [starter app type: Consumer/DeFi/Agent]
Here's the code that's failing: [paste code snippet]
What's the correct way to fix this?"

```

**Useful resources:**

-   Check your starter app’s TypeScript configuration
-   Look at how existing components in your starter app handle types
-   Different starter apps may have different type patterns for crypto functionality
-   [TypeScript documentation](https://www.typescriptlang.org/docs/) for general TypeScript issues

### Package installation issues

Dependency Conflicts

**Problem:** npm install fails with peer dependency warnings**General debugging approach:**

1.  **Read the warning carefully** - Peer dependency warnings often suggest solutions
2.  **Check your starter app’s documentation** - It may have specific setup instructions
3.  **Try standard npm troubleshooting** - Clear cache and reinstall dependencies

**Debugging with AI:**

```
"I'm getting this npm install error: [paste exact error]
Starter app type: [Consumer/DeFi/Agent]
What's the best way to resolve this dependency conflict?"

```

**Useful resources:**

-   Check your starter app’s installation instructions
-   Different starter apps may have different dependency management approaches

OnchainKit Component Issues

**Problem:** OnchainKit components not working after AI generates code**Debugging steps:**

1.  **Check your package.json** - See which OnchainKit version you’re using
2.  **Look at existing components** - See how OnchainKit is used in your starter app
3.  **Verify imports** - Make sure you’re importing from the correct OnchainKit modules
4.  **Check component documentation** - APIs may vary between versions

**Debugging with AI:**

```
"My OnchainKit component isn't working: [paste error]
OnchainKit version: [check package.json]
Starter app type: [Consumer/DeFi/Agent]
Here's my component code: [paste code]
What's the correct way to use this component?"

```

**Useful resources:**

-   [OnchainKit Documentation](https://docs.base.org/onchainkit/getting-started)
-   [OnchainKit Components](https://docs.base.org/onchainkit/components)
-   Check your starter app’s existing OnchainKit usage patterns

## Runtime errors

### Wallet connection issues

Wallet Connection Fails

**Problem:** Users can’t connect their wallet**General debugging approach:**

1.  **Check browser console** - Look for connection errors or warnings
2.  **Test with Coinbase Wallet** - Ensure Coinbase Wallet is installed and working
3.  **Verify network settings** - Make sure your app and wallet are on the same network
4.  **Test on different browsers** - Try Chrome, Safari, or Firefox

**Common solutions:**

-   **HTTPS required** - Wallet connections don’t work on HTTP in production
-   **Network mismatch** - Ensure wallet is on the correct chain (Base, Ethereum, etc.)
-   **Browser compatibility** - Make sure your browser supports wallet connections
-   **Clear cache** - Try clearing browser cache and refreshing the page

**Debugging with AI:**

```
"My wallet connection is failing. I'm using Coinbase Wallet on [network]. 
Here's the error I see in console: [paste error]
I'm building a [consumer/defi/agent] app with [starter app type]."

```

**Useful resources:**

-   Check your starter app’s documentation for wallet connection patterns
-   [OnchainKit Wallet Documentation](https://docs.base.org/onchainkit/wallet/introduction)

Transaction Failures

**Problem:** Transactions fail or get stuck**General debugging approach:**

1.  **Check transaction details** - Look at the error message for specific reasons
2.  **Verify gas settings** - Make sure you have enough ETH for gas fees
3.  **Check network status** - Network congestion can cause delays
4.  **Test with small amounts** - Start with minimal values for testing

**Common transaction errors:**

-   **User rejected** - User cancelled the transaction in their wallet
-   **Insufficient funds** - Not enough ETH for gas or tokens for transfer
-   **Network timeout** - Transaction took too long (try increasing gas)
-   **Invalid parameters** - Check recipient address and amounts

**Debugging with AI:**

```
"My transaction is failing with this error: [paste exact error]
I'm trying to [describe what you're doing]
Network: [Base/Ethereum/etc]
Transaction details: [amount, recipient, etc]"

```

**General debugging steps:**

1.  Add console.log statements to track transaction progress
2.  Check wallet for pending transactions
3.  Verify contract addresses and function parameters
4.  Test on testnet first with fake tokens

**Useful resources:**

-   Check your starter app’s transaction patterns
-   Use block explorers (Basescan, Etherscan) to inspect failed transactions

### API integration issues

CDP API Authentication

**Problem:** CDP API calls return 401 Unauthorized**Check these items:**

1.  **API keys are correctly set** in environment variables
2.  **Environment variable names** match exactly (case-sensitive)
3.  **API keys are valid** and not expired
4.  **Project ID** matches your CDP project

**General debugging approach:**

1.  **Use the CDP SDK** instead of raw API calls when possible
2.  **Check environment variables** - Print them (safely) to verify they’re loaded
3.  **Test with a simple API call** first
4.  **Read the response** - 401 errors often include helpful details

**Debugging with AI:**

```
"I'm getting a 401 error when calling CDP APIs.
Here's my error: [paste exact error message]
I'm using [CDP SDK/raw API calls]
My environment variables are: CDP_API_KEY_ID=[first few chars]..."

```

**Useful resources:**

-   [CDP Authentication Guide](https://developer.chrome.com/api-reference/v2/authentication)
-   [CDP SDK Documentation](https://developer.chrome.com/sdks/cdp-sdks-v2/typescript)
-   Use the CDP SDK for automatic JWT generation

Rate Limiting Issues

**Problem:** API calls fail with 429 Too Many Requests**General debugging approach:**

1.  **Reduce API call frequency** - Space out your requests
2.  **Check rate limits** - Review your API plan’s rate limits
3.  **Implement retry logic** - Wait before retrying failed requests
4.  **Use caching** - Avoid unnecessary duplicate API calls

**Debugging with AI:**

```
"I'm getting 429 rate limiting errors when calling CDP APIs.
API calls per minute: [estimate]
Starter app type: [Consumer/DeFi/Agent]
What's the best retry strategy for CDP APIs?"

```

**Useful resources:**

-   [CDP API Documentation](https://developer.chrome.com/api-reference/v2/introduction) for rate limit details
-   Check your starter app’s API usage patterns

## AI-generated code issues

### Code quality problems

AI Generated Insecure Code

**Problem:** AI generates code with security vulnerabilities**Security review checklist:**

-   Input validation is present
-   API keys are not hardcoded
-   User inputs are sanitized
-   Error messages don’t leak sensitive information
-   Rate limiting is implemented for API calls

**Debugging with AI:**

```
"Can you review this code for security issues?
[paste code snippet]
I'm building a [consumer/defi/agent] app.
Are there any security vulnerabilities I should fix?"

```

**Useful resources:**

-   [CDP Security Documentation](https://developer.chrome.com/get-started/authentication/security-best-practices)
-   Check your starter app’s security patterns
-   Consider security code reviews for production apps

Performance Issues

**Problem:** AI-generated code is slow or inefficient**General debugging approach:**

1.  **Identify the bottleneck** - Use browser dev tools to find slow operations
2.  **Check network requests** - Look for unnecessary API calls or large responses
3.  **Review rendering** - See if components are re-rendering too often
4.  **Test on different devices** - Performance varies across devices

**Debugging with AI:**

```
"My app is running slowly. Here's what I've observed:
- Slow operation: [describe what's slow]
- When it happens: [user action that triggers slowness]
- Starter app type: [Consumer/DeFi/Agent]
- Code that might be causing issues: [paste relevant code]
What are common performance optimizations for crypto apps?"

```

**Useful resources:**

-   Check your starter app’s performance optimization patterns
-   Use React DevTools or browser performance tools for analysis

### Logic errors

Incorrect Blockchain Interactions

**Problem:** AI generates incorrect smart contract interactions**General debugging approach:**

1.  **Test on testnet first** - Always verify logic with test tokens
2.  **Check contract ABI** - Ensure function signatures match
3.  **Validate parameters** - Verify data types and formats
4.  **Use block explorer** - Check actual transaction details on Basescan or Etherscan

**Debugging with AI:**

```
"My smart contract interaction is failing:
Error: [paste exact error]
Contract: [contract name/address]
Function: [function name]
Parameters: [list parameters]
Network: [testnet/mainnet]
What's likely wrong with this contract call?"

```

**Useful resources:**

-   Check your starter app’s smart contract interaction patterns
-   Use block explorers to verify contract addresses and ABIs
-   Test all interactions on testnet before mainnet

## Environment-specific issues

### Local development

HTTPS Requirements

**Problem:** Wallet connections fail in local development**General debugging approach:**

1.  **Check if HTTPS is required** - Most wallet connections need HTTPS
2.  **Look for HTTPS solutions** - Check your starter app’s development setup
3.  **Test on different browsers** - Some browsers have different requirements
4.  **Check console errors** - Look for specific security or connection errors

**Debugging with AI:**

```
"My wallet connections fail in local development.
Error: [paste console error]
Starter app type: [Consumer/DeFi/Agent]
Development setup: [describe your local setup]
How do I enable HTTPS for local development?"

```

**Useful resources:**

-   Check your starter app’s local development documentation
-   Different starter apps may have different HTTPS setup methods

Environment Variables Not Loading

**Problem:** Environment variables are undefined in development**General debugging approach:**

1.  **Check your starter app’s .env.example** - This shows the correct variable names and format
2.  **Verify file name** - Most Next.js apps use `.env.local` for local development
3.  **Restart development server** - Environment variables are loaded at startup
4.  **Check for syntax errors** - No spaces around `=` signs, proper quotes

**Debugging with AI:**

```
"My environment variables aren't loading. Here's my setup:
- File name: [.env.local or .env]
- Variables I'm trying to use: [list them]
- Starter app type: [Consumer/DeFi/Agent]
- Error I'm seeing: [paste error]
What's the correct format for my starter app?"

```

**Useful resources:**

-   Check your starter app’s .env.example file for the correct variable names and patterns
-   Different starter apps may use different environment variable naming conventions

### Production issues

Vercel Deployment Failures

**Problem:** App builds locally but fails on Vercel**General debugging approach:**

1.  **Check build logs** in Vercel dashboard for specific errors
2.  **Verify environment variables** are set correctly in Vercel settings
3.  **Test production build locally** - Check your package.json for build scripts
4.  **Check for differences** between local and production environments

**Debugging with AI:**

```
"My Vercel deployment is failing. Here's the error from build logs:
[paste exact error from Vercel dashboard]
Starter app type: [Consumer/DeFi/Agent]
Build works locally: [yes/no]
Environment variables set in Vercel: [list them]
What's causing this deployment failure?"

```

**Useful resources:**

-   [Vercel Documentation](https://vercel.com/docs) for deployment troubleshooting
-   Check your starter app’s deployment documentation

## Getting help

### AI debugging prompts

When asking AI for help with errors, provide this context for better results:

```
"I'm getting this error in my CDP app:
[paste exact error message]
Here's the relevant code:
[paste code snippet]
Context:
- Starter app type: [Consumer/DeFi/Agent]
- What I was trying to do: [describe the action]
- When it happens: [during build/runtime/etc]
- Already tried: [list what you've attempted]
What's the most likely cause and how do I fix it?"

```

-   **[CDP Discord](https://discord.gg/cdp)**: Real-time community support
-   **[GitHub Discussions](https://github.com/orgs/coinbase/discussions)**: Technical Q&A
-   **[OnchainKit Issues](https://github.com/coinbase/onchainkit/issues)**: Component-specific issues

## What to read next

-   **[AI Prompting Techniques](https://developer.chrome.com/get-started/develop-with-ai/development/ai-prompting-techniques)**: Effective AI interaction patterns
-   **[Deployment Guide](https://developer.chrome.com/get-started/develop-with-ai/development/ai-deployment)**: Smooth deployment workflows
-   **[Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Advanced patterns
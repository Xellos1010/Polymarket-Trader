# ai deployment

## Overview

This guide walks you through deploying your crypto application with AI assistance. While there are many deployment platforms available (like Netlify, Railway, Render, or AWS), this guide focuses on Vercel as a popular option for web applications. **Why use Vercel?**

-   **Automatic deployments**: Just push to GitHub and your app goes live instantly (great for rapid iteration and sharing with others)
-   **Global performance**: Your app loads fast worldwide through their CDN
-   **Built-in HTTPS**: Secure connections for handling crypto transactions
-   **Preview deployments**: Test changes safely before they go live
-   **Easy environment variables**: Secure storage for your API keys

You’ll learn how to set up automatic deployments, configure environment variables securely, and troubleshoot common issues using Vercel.

## Environment variables

Based on your app type, you’ll need different environment variables. Check your `.env` file from your scaffolded project for the variables you need.

### Why use Vercel environment variables

-   **Security**: Your secrets are encrypted and stored safely (not in your code where others could see them)
-   **Flexibility**: Use different API keys for testing vs production
-   **Convenience**: Vercel automatically adds these variables to your app when it builds
-   **Team-friendly**: Share projects without sharing secret files

### Best practices

-   **Keep secrets private**: Only use `NEXT_PUBLIC_` prefix if the variable needs to be visible in the browser (most API keys should NOT have this prefix)
-   **Use Vercel’s dashboard**: Add variables through the Vercel interface rather than putting them in code
-   **Update keys regularly**: Change your API keys periodically for security
-   **Rebuild after changes**: If you change environment variables, trigger a new deployment so they take effect

For more details, see [Vercel’s environment variables documentation](https://vercel.com/docs/environment-variables) and [sensitive environment variables guide](https://vercel.com/docs/environment-variables/sensitive-environment-variables).

## Vercel deployment

If your repository is pushed to version control ([GitHub](https://docs.github.com/en/get-started/quickstart/create-a-repo), [GitLab](https://docs.gitlab.com/ee/user/project/repository/), or [Bitbucket](https://support.atlassian.com/bitbucket-cloud/docs/create-a-repository/)), you can import the repository to Vercel, a tool used for hosting and deploying web applications with automatic builds and deployments.

### Prerequisites

Before deploying to Vercel, make sure you have:

-   **Existing app integrated with CDP** - Follow the [starter app setup guide](https://developer.chrome.com/get-started/develop-with-ai/setup/ai-development-setup) if you haven’t created one yet
-   **Repository pushed to version control** - Your code should be in GitHub, GitLab, or Bitbucket

### Initial setup

## Troubleshooting with AI

When deployments fail, you can use AI to help diagnose and fix issues quickly.

### Build failures

When Vercel can’t build your app (you’ll see errors in the deployment log):

```
"My Vercel deployment is failing:
Error: [paste build error]
App type: [embedded wallet/DeFi/agent]
Recent changes: [what you added]
Help me fix the build and deploy successfully."

```

### Environment variable issues

When your app works locally but can’t find your API keys after deployment:

```
"Environment variable problems in production:
Variables set: [list what you configured]
Error: [paste error message]
Expected behavior: [what should work]
Debug my environment configuration and suggest fixes."

```

### Network connection problems

When your app works locally but can’t connect to blockchain networks after deployment:

```
"Network issues after deployment:
Error: [paste error]
Network: [Base/Ethereum]
Local works: [yes/no]
Help debug production network configuration."

```

## Common fixes with AI help

### Code style errors (linting)

First, try to auto-fix common code style issues:

```
npm run lint
npm run lint -- --fix  # Automatically fixes many common issues

```

If there are still errors, ask AI for help:

```
"Fix these lint errors in my crypto app:
Errors: [paste errors]
Code: [paste relevant files]
Keep crypto-specific patterns like BigInt usage."

```

### Slow loading apps

If your deployed app feels slow, ask AI to help optimize it:

```
"Optimize my crypto app build for production:
Current bundle size: [size]
Performance issues: [describe]
Target: faster loading, smaller bundle
Include: OnchainKit optimization, dynamic imports, asset optimization."

```

## What to read next

-   **[AI Debugging](https://developer.chrome.com/get-started/develop-with-ai/development/ai-debugging)**: Debug production issues with AI assistance and systematic troubleshooting
-   **[Testing Strategies](https://developer.chrome.com/get-started/develop-with-ai/development/ai-testing)**: Ensure quality before deployment with comprehensive testing approaches
-   **[AI Development Workflows](https://developer.chrome.com/get-started/develop-with-ai/development/develop-with-ai-workflows)**: Core development patterns and workflow processes
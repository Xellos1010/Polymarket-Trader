# hosting agents

There are several options for hosting your AgentKit agents, from managed AI agent platforms to traditional cloud hosting. This guide covers the main approaches and helps you choose the right one for your needs.

-   Autonome
    
-   NEAR AI
    
-   Cloud Hosting
    

[Autonome](https://apps.autono.meme/) by AltLayer provides a managed platform specifically designed for hosting crypto AI agents. It offers:

-   Simple deployment process
-   Built-in monitoring and management
-   Team collaboration features
-   Support for multiple agent frameworks including AgentKit

#### Quick Setup

1.  Visit [apps.autono.meme](https://apps.autono.meme/)
2.  Log in and create an organization
3.  Click ”+” to deploy a new agent
4.  Select “AgentKit” as your framework
5.  Enter required API keys (OpenAI, CDP, etc.)
6.  Deploy your agent

For detailed instructions, see the [Autonome deployment guide](https://docs.altlayer.io/altlayer-documentation/autonome/deploy-ai-agent).

#### Custom Framework Upload

If you have a custom AgentKit implementation, you can upload it as a new framework:

1.  Package your agent as a Docker image
2.  Click “Upload New Framework” in Autonome
3.  Configure the framework details including:
    -   Framework name
    -   Description
    -   Docker image URL
    -   Chat endpoint and port
    -   Environment variables

See the [framework upload guide](https://docs.altlayer.io/altlayer-documentation/autonome/uploading-your-own-agent-framework) for complete details.

[NEAR AI](https://near.ai/) provides a platform for building and hosting AI agents with built-in support for various frameworks including AgentKit.

#### Requirements

-   Python 3.9 - 3.11 (3.12 - 3.13 is not supported)
-   NEAR Account (can be created at [wallet.near.org](https://wallet.near.org/))
-   Virtual environment (recommended)

#### Quick Setup

1.  Create and activate a Python virtual environment:

```
# Using venv
python -m venv .venv
source .venv/activate
# Or using conda
conda create -n nearai python=3.11
conda activate nearai

```

2.  Install the NEAR AI CLI:

3.  Login to NEAR AI:

4.  Create a new agent:

5.  Configure your agent’s framework in `metadata.json`:

```
{
  "details": {
    "agent": {
      "framework": "agentkit"
    }
  }
}

```

For detailed setup instructions, see:

-   [NEAR AI CLI Guide](https://docs.near.ai/cli/)
-   [NEAR AI Quickstart Guide](https://docs.near.ai/agents/quickstart/)
-   [Framework Support Documentation](https://docs.near.ai/agents/env/frameworks/)

You can host your AgentKit agent on any major cloud platform. Here’s a general approach:

#### AWS Setup

1.  Create an ECS cluster or EKS cluster
2.  Build and push your agent’s Docker image to ECR
3.  Deploy using ECS tasks or Kubernetes deployments

#### Google Cloud Setup

1.  Create a GKE cluster or Cloud Run service
2.  Push your Docker image to Container Registry
3.  Deploy using Cloud Run or Kubernetes

## Choosing a Hosting Solution

Consider these factors when selecting a hosting solution:

-   **Managed Platforms** (Autonome, NEAR AI): Best for quick deployment and managed infrastructure
-   **Cloud Hosting**: Ideal for custom requirements and full control over infrastructure
-   **Local Hosting**: Perfect for development and testing

For most users, managed platforms like Autonome or NEAR AI provide the easiest path to production. For enterprise needs or specific requirements, traditional cloud hosting offers more control and customization options.
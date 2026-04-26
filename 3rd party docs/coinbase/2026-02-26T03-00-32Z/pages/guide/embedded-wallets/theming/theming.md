# theming

## Overview

The library comes with a comprehensive theming system that allows you to customize the look and feel of all components to match your brand. The theme is built on a small set of core **semantic tokens** that control foundational styles like colors and typography. These are inherited by more specific **component tokens** used to style individual UI elements. This page provides a complete reference for all available theme tokens. For a guide on how to apply a theme, see the [React Components](https://developer.chrome.com/embedded-wallets/react-components#3-customize-theme-optional) page.

## Semantic Tokens

Semantic tokens are the foundational variables for the theme. By customizing this small set of tokens, you can quickly and easily apply a new theme across all components.

Token

Description

Default value

**Backgrounds**

`colors-bg-default`

Default page and input background

`#ffffff`

`colors-bg-alternate`

Alternate backgrounds (e.g. cards)

`#eef0f3`

`colors-bg-contrast`

A contrast color mixed with other backgrounds to generate default hover and pressed states.

[`colors-fg-default`](#colors-fg-default)

`colors-bg-overlay`

Overlay UI (e.g. modals).

[`colors-bg-alternate`](#colors-bg-alternate) (33% alpha)

`colors-bg-skeleton`

Loading placeholders.

[`colors-fg-default`](#colors-fg-default) (10% alpha)

`colors-bg-primary`

Primary brand background (e.g. CTA)

`#0052ff`

`colors-bg-secondary`

Secondary background (e.g. secondary CTA)

`#eef0f3`

**Text**

`colors-fg-default`

Default text color

`#0a0b0d`

`colors-fg-muted`

Muted/placeholder text

`#5b616e`

`colors-fg-primary`

Primary action text (e.g. links)

`#0052ff`

`colors-fg-onPrimary`

Text on primary backgrounds

`#ffffff`

`colors-fg-onSecondary`

Text on secondary backgrounds

`#0a0b0d`

`colors-fg-positive`

Success messaging text

`#098551`

`colors-fg-negative`

Error messaging text

`#cf202f`

`colors-fg-warning`

Warning messaging text

`#ed702f`

**Borders**

`colors-line-default`

Standard borders (e.g. dividers)

`#dcdfe4`

`colors-line-heavy`

Input borders

`#9397a0`

`colors-line-primary`

Focus/active borders

[`colors-fg-primary`](#colors-fg-primary)

`colors-line-positive`

Success state borders

[`colors-fg-positive`](#colors-fg-positive)

`colors-line-negative`

Error borders

[`colors-fg-negative`](#colors-fg-negative)

**Typography**

`font-family-sans`

Sans-serif font family

`"Rethink Sans", -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, Helvetica, Arial, sans-serif, "Apple Color Emoji", "Segoe UI Emoji", "Segoe UI Symbol"`

`font-family-mono`

Monospace font family

`"DM Mono", monospace`

`font-size-base`

Base font size

`16`

## Component Tokens

Component tokens can be overridden individually for more fine-grained control over your theme. While most theming can be accomplished by updating the semantic tokens, these component-level overrides allow for more specific customizations.

Token

Description

Default value

**Page**

`page-bg-default`

Page background color

[`colors-bg-default`](#colors-bg-default)

`page-border-default`

Page border color

[`colors-line-default`](#colors-line-default)

`page-text-default`

Page text color

[`colors-fg-default`](#colors-fg-default)

`page-text-muted`

Page muted text color

[`colors-fg-muted`](#colors-fg-muted)

**CTA Primary**

`cta-primary-bg-default`

Primary CTA background color

[`colors-bg-primary`](#colors-bg-primary)

`cta-primary-bg-hover`

Primary CTA hover background color

[`colors-bg-primary`](#colors-bg-primary) mixed with 15% [`colors-bg-contrast`](#colors-bg-contrast)

`cta-primary-bg-pressed`

Primary CTA pressed background color

[`colors-bg-primary`](#colors-bg-primary) mixed with 30% [`colors-bg-contrast`](#colors-bg-contrast)

`cta-primary-border-focus`

Primary CTA focus border color

[`colors-line-primary`](#colors-line-primary)

`cta-primary-text-default`

Primary CTA text color

[`colors-fg-onPrimary`](#colors-fg-onPrimary)

`cta-primary-text-hover`

Primary CTA hover text color

[`colors-fg-onPrimary`](#colors-fg-onPrimary)

**CTA Secondary**

`cta-secondary-bg-default`

Secondary CTA background color

[`colors-bg-secondary`](#colors-bg-secondary)

`cta-secondary-bg-hover`

Secondary CTA hover background color

[`colors-bg-secondary`](#colors-bg-secondary) mixed with 10% [`colors-bg-contrast`](#colors-bg-contrast)

`cta-secondary-bg-pressed`

Secondary CTA pressed background color

[`colors-bg-secondary`](#colors-bg-secondary) mixed with 20% [`colors-bg-contrast`](#colors-bg-contrast)

`cta-secondary-border-focus`

Secondary CTA focus border color

[`colors-line-primary`](#colors-line-primary)

`cta-secondary-text-default`

Secondary CTA text color

[`colors-fg-onSecondary`](#colors-fg-onSecondary)

`cta-secondary-text-hover`

Secondary CTA hover text color

[`colors-fg-onSecondary`](#colors-fg-onSecondary)

**Link Primary**

`link-primary-text-default`

Primary link text color

[`colors-fg-primary`](#colors-fg-primary)

`link-primary-text-hover`

Primary link hover text color

[`colors-fg-primary`](#colors-fg-primary) mixed with 15% [`colors-bg-contrast`](#colors-bg-contrast)

`link-primary-text-pressed`

Primary link pressed text color

[`colors-fg-primary`](#colors-fg-primary) mixed with 30% [`colors-bg-contrast`](#colors-bg-contrast)

**Link Secondary**

`link-secondary-text-default`

Secondary link text color

[`colors-fg-default`](#colors-fg-default)

`link-secondary-text-hover`

Secondary link hover text color

[`colors-fg-default`](#colors-fg-default) mixed with 10% [`colors-bg-contrast`](#colors-bg-contrast)

`link-secondary-text-pressed`

Secondary link pressed text color

[`colors-fg-default`](#colors-fg-default) mixed with 20% [`colors-bg-contrast`](#colors-bg-contrast)

**Input**

`input-bg-default`

Input background color

[`colors-bg-default`](#colors-bg-default)

`input-border-default`

Input border color

[`colors-line-heavy`](#colors-line-heavy)

`input-border-focus`

Input focus border color

[`colors-line-primary`](#colors-line-primary)

`input-border-error`

Input error border color

[`colors-line-negative`](#colors-line-negative)

`input-border-success`

Input success border color

[`colors-line-positive`](#colors-line-positive)

`input-label-default`

Input label text color

[`colors-fg-default`](#colors-fg-default)

`input-placeholder-default`

Input placeholder text color

[`colors-fg-muted`](#colors-fg-muted)

`input-text-default`

Input text color

[`colors-fg-default`](#colors-fg-default)

`input-errorText-default`

Input error text color

[`colors-fg-negative`](#colors-fg-negative)

`input-successText-default`

Input success text color

[`colors-fg-positive`](#colors-fg-positive)

**Select**

`select-label-default`

Select label text color

[`colors-fg-default`](#colors-fg-default)

`select-trigger-bg-default`

Select trigger background color

[`colors-bg-default`](#colors-bg-default)

`select-trigger-bg-hover`

Select trigger hover background color

[`colors-bg-default`](#colors-bg-default) mixed with 5% [`colors-bg-contrast`](#colors-bg-contrast)

`select-trigger-bg-pressed`

Select trigger pressed background color

[`colors-bg-default`](#colors-bg-default) mixed with 7% [`colors-bg-contrast`](#colors-bg-contrast)

`select-trigger-border-default`

Select trigger border color

[`colors-line-default`](#colors-line-default)

`select-trigger-border-focus`

Select trigger focus border color

[`colors-line-primary`](#colors-line-primary)

`select-trigger-border-error`

Select trigger error border color

[`colors-line-negative`](#colors-line-negative)

`select-trigger-border-success`

Select trigger success border color

[`colors-line-positive`](#colors-line-positive)

`select-trigger-placeholder-default`

Select trigger placeholder text color

[`colors-fg-muted`](#colors-fg-muted)

`select-trigger-text-default`

Select trigger text color

[`colors-fg-default`](#colors-fg-default)

`select-trigger-errorText-default`

Select trigger error text color

[`colors-fg-negative`](#colors-fg-negative)

`select-trigger-successText-default`

Select trigger success text color

[`colors-fg-positive`](#colors-fg-positive)

`select-list-bg-default`

Select list background color

[`colors-bg-default`](#colors-bg-default)

`select-list-border-default`

Select list border color

[`colors-line-default`](#colors-line-default)

`select-list-border-focus`

Select list focus border color

[`colors-line-primary`](#colors-line-primary)

`select-list-border-error`

Select list error border color

[`colors-line-negative`](#colors-line-negative)

`select-list-border-success`

Select list success border color

[`colors-line-positive`](#colors-line-positive)

`select-list-item-bg-default`

Select list item background color

[`colors-bg-default`](#colors-bg-default)

`select-list-item-bg-highlight`

Select list item highlight background color

[`colors-bg-default`](#colors-bg-default) mixed with 10% [`colors-bg-contrast`](#colors-bg-contrast)

`select-list-item-text-default`

Select list item text color

[`colors-fg-default`](#colors-fg-default)

`select-list-item-text-muted`

Select list item muted text color

[`colors-fg-muted`](#colors-fg-muted)

`select-list-item-text-onHighlight`

Select list item highlighted text color

[`colors-fg-default`](#colors-fg-default)

`select-list-item-text-mutedOnHighlight`

Select list item highlighted muted text color

[`colors-fg-muted`](#colors-fg-muted)

**Code**

`code-bg-default`

Code block background color

[`colors-bg-alternate`](#colors-bg-alternate)

`code-border-default`

Code block border color

[`colors-line-heavy`](#colors-line-heavy)

`code-text-default`

Code block text color

[`colors-fg-default`](#colors-fg-default)

## What to read next

-   [**CDP Web SDK Documentation**](https://developer.chrome.com/sdks/cdp-sdks-v2/frontend): Comprehensive API reference for the CDP Web SDK
-   **[Authentication Methods](https://developer.chrome.com/embedded-wallets/authentication-methods)**: Learn about available authentication options
-   **[Implementation Guide](https://developer.chrome.com/embedded-wallets/implementation-guide)**: Step-by-step authentication integration
-   [**React Hooks**](https://developer.chrome.com/embedded-wallets/react-hooks): Learn about available hooks like `useSignInWithEmail`, `useEvmAddress`, and `useSendEvmTransaction`
-   [**React Components**](https://developer.chrome.com/embedded-wallets/react-components): Explore pre-built components for authentication, wallet management, and transactions
-   [**React Native**](https://developer.chrome.com/embedded-wallets/react-native): Build mobile apps with Coinbase Developer Platform (CDP) embedded wallets
-   [**Wagmi Integration**](https://developer.chrome.com/embedded-wallets/wagmi): Use CDP wallets with the popular wagmi library for Ethereum development
-   [**Security & Export**](https://developer.chrome.com/embedded-wallets/security-export): Learn about private key export security considerations and implementation
# Contributing to Paperplane

Thank you for your interest in contributing to Paperplane. This document
defines the standards and workflow to ensure consistency,
maintainability, and high code quality across the project.

------------------------------------------------------------------------

## 1. Principles

-   Prioritize working software over theoretical design.
-   Keep implementations simple and maintainable.
-   Optimize for performance in the Rust core.
-   Prefer incremental improvements over large rewrites.
-   Avoid introducing unnecessary abstractions.

------------------------------------------------------------------------

## 2. Project Structure Overview

    apps/
      desktop/     Desktop application (Tauri + React)
      cli/         CLI runner (planned)

    rust-core/     Core execution engine

    packages/
      ui/          Shared UI components
      core/        Request orchestration logic
      storage/     File and persistence layer
      env/         Environment handling
      testing/     Assertions and scripting

------------------------------------------------------------------------

## 3. Branching Strategy

-   main: Stable, production-ready code
-   develop: Integration branch for ongoing work
-   feature/\*: New features
-   fix/\*: Bug fixes
-   chore/\*: Tooling or maintenance tasks

Rules: - Do not commit directly to main - All changes must go through
pull requests - Keep branches short-lived and focused

------------------------------------------------------------------------

## 4. Commit Conventions

All commits must follow the Conventional Commits format:

    <type>: <short description>

Examples:

    feat: implement GET request execution
    fix: handle timeout error in HTTP client
    refactor: simplify request serialization logic
    chore: update dependencies
    docs: update contributing guidelines

Allowed types:

-   feat: New feature
-   fix: Bug fix
-   refactor: Code change without functional change
-   chore: Maintenance or tooling
-   docs: Documentation updates

Rules: - Use present tense - Keep subject line under 72 characters - One
logical change per commit

------------------------------------------------------------------------

## 5. Issue Workflow

Every piece of work must be tracked via an issue.

Each issue should include: - Clear description - Defined scope -
Acceptance criteria - Appropriate labels

Status flow:

Backlog → Ready → In Progress → Done

Only work on issues assigned to the current iteration.

------------------------------------------------------------------------

## 6. Pull Request Guidelines

### Title Format

    <type>: <short description>

Example:

    feat: add POST request support

### Description Template

    ## Summary
    Describe the purpose of this PR.

    ## Changes
    - List key changes

    ## Testing
    Explain how this was tested.

    ## Notes
    Any additional context or constraints

### Requirements

Before submitting a PR:

-   Code builds successfully
-   No runtime errors
-   No unused code or dead logic
-   Changes are scoped to a single concern
-   Linked to an existing issue

### Size Guidelines

-   Preferred: under 300 lines
-   Maximum: 500 lines

If larger, split into multiple PRs.

------------------------------------------------------------------------

## 7. Code Quality Standards

-   Write readable and self-explanatory code
-   Avoid deeply nested logic
-   Prefer explicit over implicit behavior
-   Remove commented-out code before submission
-   Ensure consistent formatting

------------------------------------------------------------------------

## 8. Rust Core Guidelines

-   Use async patterns correctly
-   Avoid blocking operations
-   Minimize memory allocations where possible
-   Return structured errors (Result types)
-   Keep request execution path efficient

------------------------------------------------------------------------

## 9. Frontend Guidelines

-   Keep components small and reusable
-   Avoid unnecessary state duplication
-   Use centralized state management
-   Handle loading and error states explicitly
-   Do not hardcode values that should be configurable

------------------------------------------------------------------------

## 10. Testing Approach

-   Prioritize testing for core execution logic
-   Ensure error paths are covered
-   UI testing can be minimal during MVP phase
-   Manual verification is acceptable for early development

------------------------------------------------------------------------

## 11. Definition of Done

A task is considered complete when:

-   Functionality works end-to-end
-   No crashes or critical bugs
-   Code follows project conventions
-   Acceptance criteria are satisfied

------------------------------------------------------------------------

## 12. What to Avoid

-   Over-engineering early-stage features
-   Introducing unused abstractions
-   Combining multiple features in one PR
-   Making breaking changes without discussion
-   Adding dependencies without clear justification

------------------------------------------------------------------------

## 13. Getting Started

1.  Pick an issue from the current iteration
2.  Create a branch from develop
3.  Implement the change
4.  Commit using the defined conventions
5.  Open a pull request
6.  Address feedback and merge

------------------------------------------------------------------------

## 14. License

By contributing, you agree that your contributions will be licensed
under the MIT License.

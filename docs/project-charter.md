# LocalID Project Charter

**Version:** 0.1  
**Status:** Draft

## Vision

LocalID is a self-hosted identity platform designed to provide reliable identity, authentication, and session management for personal, homelab, and small-scale systems.

The project prioritizes correctness, maintainability, and long-term evolution over rapid feature development.

## Mission

Build an identity platform that developers can confidently understand, extend, operate, and maintain over the long term without unnecessary complexity.

## Goals

LocalID aims to provide:

- identity management;
- credential management;
- authentication;
- session management;
- client application management;
- clear domain boundaries;
- modular architecture;
- high testability;
- reliable security foundations;
- self-hosted deployment.

## Non-Goals

LocalID is not initially intended to become:

- an enterprise identity and access management platform;
- a replacement for Active Directory;
- a replacement for Keycloak;
- a cloud identity service;
- a Kubernetes authentication platform;
- a complete authorization framework;
- a general-purpose user management application.

## Target Users

LocalID is primarily designed for:

- self-hosting enthusiasts;
- homelab operators;
- independent developers;
- personal applications;
- small internal systems.

It is not initially optimized for large enterprise deployments.

## Core Values

### Domain First

Business concepts and rules define the architecture. Technology choices follow domain requirements.

### Correctness Before Convenience

Correct and predictable behavior is preferred over implementation convenience.

### Explicit Design

Responsibilities, state transitions, errors, and boundaries should be visible and intentional.

### Modularity

Each component should have a focused responsibility and a small public interface.

### Technology Independence

Transport, storage, frameworks, and deployment models are implementation details.

### Documentation as Product

Important terminology, boundaries, and architectural decisions are maintained alongside the source code.

### Long-Term Maintainability

Readability and maintainability are preferred over cleverness and premature optimization.

## Success Criteria

LocalID is successful when:

- its domain model is understandable;
- its component boundaries remain clear;
- business rules can be tested without transport or storage;
- implementation technologies can change without redesigning the domain;
- important architectural decisions are documented;
- the system remains reliable and maintainable over time.

## Guiding Principle

> Understand first. Build second.

## Living Document

This charter may evolve as LocalID grows. Changes must reflect deliberate changes in project direction rather than temporary implementation details.

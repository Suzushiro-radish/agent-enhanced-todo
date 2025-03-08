# Agent-Enhanced Todo App - Development Guide

## Commands
- `npm run dev` - Start development server
- `npm run build` - Build the application
- `npm run check` - Type check TypeScript code
- `npm run check:watch` - Watch mode for type checking
- `npm run tauri dev` - Run Tauri app in development mode
- `npm run tauri build` - Build Tauri application

## Code Style Guidelines
- **TypeScript**: Use strict typing with interfaces for data structures
- **Components**: Follow Svelte component structure (script, markup, style)
- **Naming**: camelCase for variables/functions, PascalCase for components/types
- **Imports**: Group imports by source (built-in, external, internal)
- **Error Handling**: Use proper error handling with typed errors
- **Backend**: Follow Rust conventions for Tauri commands
- **State Management**: Use Tauri state for persistent data
- **Documentation**: Add JSDoc comments for functions
- **Frontend/Backend**: Maintain clear separation of concerns

## Project Structure
- `/src` - Svelte frontend code
- `/src-tauri` - Rust backend code
# Hugging Face Inference Status Tracker Documentation

## Overview

The Hugging Face Inference Status Tracker is a desktop application that helps users monitor and compare inference services from Hugging Face. It provides real-time information about model performance metrics such as pricing, throughput, latency, and feature support across different service providers.

## Architecture

### Frontend (Client-Side)
- **Framework**: Leptos (Rust-based reactive web framework)
- **UI Library**: Custom components built with Leptos
- **State Management**: Reactive stores using `reactive_stores` crate
- **Styling**: Tailwind CSS for responsive design

### Backend (Server-Side)
- **Runtime**: Tauri v2 (Rust-based desktop application framework)
- **Data Processing**: Polars DataFrame library for efficient data manipulation
- **HTTP Client**: Reqwest for API and web scraping
- **HTML Parsing**: Scraper crate for parsing Hugging Face inference tables
- **Data Persistence**: Tauri plugin store for local storage of favorites
- **Hashing**: Blake3 for generating unique IDs for model-service combinations

## Technology Stack

### Frontend Technologies
- **Leptos**: Modern Rust web framework with fine-grained reactivity
- **Reactive Stores**: State management solution for Leptos applications
- **Tailwind CSS**: Utility-first CSS framework for styling
- **Trunk**: Build tool for client-side WebAssembly applications

### Backend Technologies
- **Rust**: Systems programming language for performance and safety
- **Tauri**: Framework for building secure desktop applications
- **Polars**: High-performance DataFrame library for data manipulation
- **Reqwest**: HTTP client for making API requests
- **Scraper**: HTML parsing library for extracting data from web pages
- **Tokio**: Asynchronous runtime for Rust
- **Serde**: Serialization/deserialization framework
- **Chrono**: Date and time handling library

### Data Flow Technologies
- **WebAssembly (WASM)**: Enables running Rust code in the browser
- **Tauri Commands**: Bridge between frontend and backend via IPC
- **JSON**: Data interchange format for communication

## Design Patterns

### State Management Pattern
The application follows a reactive state management pattern using Leptos stores:
- Global application state is provided via `provide_context()`
- Components subscribe to state changes using reactive primitives
- State updates trigger automatic UI re-renders

### Repository Pattern
Backend implements a repository pattern for data access:
- Separates data fetching logic from business logic
- Provides clean interfaces for data operations
- Enables easy testing and maintenance

### Service Layer Pattern
Business logic is encapsulated in service layers:
- `FavoriteModelService`: Handles favorite model operations and persistence
- `InferenceModelState`: Manages the state of inference model data
- Clear separation of concerns between data access and business rules

### Observer Pattern
Components observe state changes reactively:
- Memoized computations that update when dependencies change
- Effects that run when specific state values change
- Automatic UI updates without manual DOM manipulation

## Current Workflows and Data Flows

### Main Application Flow
1. **Initialization**: App initializes state stores and fetches initial data
2. **Data Fetching**: Backend scrapes Hugging Face inference status page
3. **Data Processing**: HTML table is parsed and converted to DataFrame
4. **Data Storage**: Processed data is stored in application state
5. **UI Rendering**: Frontend renders data in tabular format
6. **User Interaction**: Users can filter, sort, and favorite models

### Data Fetching Flow
1. **Trigger**: User clicks refresh button or app initializes
2. **Backend Request**: Tauri command is invoked (`get_data`)
3. **Web Scraping**: Backend fetches HTML from Hugging Face inference page
4. **Parsing**: HTML table is parsed using CSS selectors
5. **DataFrame Creation**: Parsed data is converted to Polars DataFrame
6. **Processing**: Unique IDs are generated using Blake3 hashing
7. **Storage**: DataFrame is stored in application state
8. **Response**: Processed data is returned to frontend

### Favorite Management Flow
1. **User Action**: User clicks favorite button on a model
2. **State Update**: Frontend updates local favorite state
3. **Backend Call**: Tauri command is invoked (`add_favorite`)
4. **Persistence**: Favorite is saved to local storage via tauri-plugin-store
5. **Snapshot Creation**: Current model state is saved as a reference point
6. **UI Update**: Favorite status is reflected in the UI

### Filtering Flow
1. **Toggle Action**: User toggles "Show Favorites Only" option
2. **State Change**: Favorite state is updated to reflect user preference
3. **Data Request**: App requests filtered data from backend
4. **Backend Processing**: Backend filters DataFrame based on favorite IDs
5. **Response**: Filtered data is returned and displayed

## Planned Features and Implementation Approach

### 1. Enhanced Favorite Management
**Feature**: More sophisticated favorite management with categorization and notes.

**Implementation Approach**:
- Extend the `FavoriteModelService` to support additional metadata
- Add category and note fields to the favorite model structure
- Create UI components for managing favorite categories and notes
- Implement filtering by category in the frontend

### 2. Notification System for Changes
**Feature**: Real-time notifications when favorite model services change (pricing, throughput, latency).

**Implementation Approach**:
- Implement snapshot comparison logic in the backend
- Create a monitoring service that periodically checks for changes
- Use Tauri's notification API to display desktop notifications
- Store historical data to enable change detection
- Implement configurable alert thresholds for different metrics

### 3. Advanced Table Manipulation
**Feature**: Advanced sorting, grouping, and filtering capabilities.

**Implementation Approach**:
- Enhance the `AdvancedTable` component with sorting controls
- Implement multi-column sorting functionality
- Add dynamic grouping by multiple columns
- Create filter panels for complex queries
- Implement virtual scrolling for large datasets
- Add column customization options (show/hide, reorder)

### 4. Historical Data Tracking
**Feature**: Track and visualize changes in model performance over time.

**Implementation Approach**:
- Extend data persistence to store historical snapshots
- Implement time-series data structures
- Create visualization components for trend analysis
- Add export functionality for historical data
- Implement data retention policies

### 5. Export and Reporting
**Feature**: Export data to various formats (CSV, JSON, Excel).

**Implementation Approach**:
- Implement export functionality in the backend using Polars I/O capabilities
- Create UI controls for selecting export format and range
- Add reporting templates for common use cases
- Implement batch export for large datasets

## Key Components

### Frontend Components
- **App**: Root component that sets up global state
- **ModelInferenceStatus**: Main view component for displaying model data
- **ModelInferenceStatusList**: Component for rendering the list of models
- **AdvancedTable**: Reusable table component with custom headers
- **TableRow**: Individual row component for model data
- **InferenceStatusToolBar**: Toolbar with filtering and refresh controls
- **ToggleShowFavoriteOnly**: Toggle for showing favorite models only

### Backend Modules
- **Commands**: Tauri command handlers for frontend-backend communication
- **Models**: Data structures representing domain entities
- **Modules**: Business logic organized by feature areas
- **States**: Application state management
- **Types**: Shared type definitions
- **Repository**: Data access layer abstractions

### Data Models
- **InferenceModelStatusRowData**: Represents a single row of model inference data
- **TableColumn**: Enum for different table columns
- **InitStatus**: Status of data initialization (loading, error, etc.)

## Development Environment

### Prerequisites
- Rust (latest stable)
- Node.js and npm
- Trunk (Rust web application bundler)
- Tauri CLI

### Setup Instructions
```bash
# Install Trunk
cargo install trunk

# Install Tauri CLI
cargo install tauri-cli

# Install dependencies
npm install

# Run in development mode
cargo tauri dev
```

### Building
```bash
# Build for production
cargo tauri build
```

## Testing Strategy

The application currently has minimal test coverage. Future testing strategy should include:
- Unit tests for backend data processing functions
- Integration tests for Tauri commands
- Component tests for frontend UI elements
- End-to-end tests for critical user flows

## Performance Considerations

- Data is processed using Polars DataFrame for efficient computation
- Web scraping is performed asynchronously to avoid blocking the UI
- State updates are optimized using reactive programming principles
- Virtual scrolling could be implemented for large datasets in the future

## Security Considerations

- Desktop application with limited external API exposure
- Local data storage with tauri-plugin-store
- Input validation for all user-facing fields
- Secure web scraping practices
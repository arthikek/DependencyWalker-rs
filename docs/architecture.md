# Dependency Walker - Software Architecture

## 1. Introduction

### 1.1 Purpose
This document outlines the software architecture for the backend logic of a Dependency Walker application. The application is designed to analyze DLL files, extract their dependencies, and display them in a hierarchical structure. The backend is implemented in Rust, leveraging the Slint UI for the frontend.

### 1.2 Scope
The architecture covers all core backend components, including file handling, PE format parsing, dependency analysis, and error handling. This architecture is designed to be modular, scalable, and robust, ensuring maintainability and ease of extension.

## 2. Architectural Overview

### 2.1 High-Level Architecture

The backend architecture is divided into several key modules:

1. **File I/O Module**
2. **PE Parser Module**
3. **Dependency Analysis Module**
4. **Error Handling Module**
5. **Data Structures**

### 2.2 Data Flow

1. **User Interaction:** The user selects a DLL file via the frontend UI.
2. **File I/O Module:** The selected file is opened and read.
3. **PE Parser Module:** The PE header, import table, and export table are parsed from the DLL.
4. **Dependency Analysis Module:** A dependency tree is constructed based on the parsed data.
5. **Error Handling Module:** Any errors encountered during the process are captured and communicated to the frontend.

### 2.3 Modular Design

The backend logic is divided into discrete modules, each responsible for specific tasks. This modular design ensures that each component can be developed, tested, and maintained independently.

## 3. Detailed Architecture

### 3.1 File I/O Module

#### 3.1.1 Responsibilities
- Open and read DLL files from the file system.
- Provide the raw binary data to the PE Parser Module.

#### 3.1.2 Interfaces
- `open_file(path: &str) -> Result<File, Error>`
    - Opens the specified file and returns a file handle.

- `read_bytes(file: &mut File, offset: u64, length: usize) -> Result<Vec<u8>, Error>`
    - Reads a specified number of bytes from the file at the given offset.

#### 3.1.3 Error Handling
- If the file cannot be opened or read, an `Error::FileNotFound` or `Error::ReadError` is returned to the Error Handling Module.

### 3.2 PE Parser Module

#### 3.2.1 Responsibilities
- Parse the Portable Executable (PE) header.
- Extract the Import Table and Export Table.
- Provide structured data to the Dependency Analysis Module.

#### 3.2.2 Interfaces
- `parse_pe_header(file: &mut File) -> Result<PEHeader, Error>`
    - Parses the PE header from the provided file.

- `parse_import_table(pe_header: &PEHeader, file: &mut File) -> Result<Vec<ImportFunction>, Error>`
    - Parses the Import Table from the DLL file based on the PE header information.

- `parse_export_table(pe_header: &PEHeader, file: &mut File) -> Result<Vec<ExportFunction>, Error>`
    - Parses the Export Table from the DLL file.

#### 3.2.3 Data Structures
- **PEHeader**
    - Stores metadata from the PE header.

- **ImportFunction**
    - Represents an imported function, including the function name, ordinal, and hint.

- **ExportFunction**
    - Represents an exported function, including the function name, ordinal, and address.

#### 3.2.4 Error Handling
- If the PE header or tables cannot be parsed, an `Error::ParseError` is returned.

### 3.3 Dependency Analysis Module

#### 3.3.1 Responsibilities
- Analyze the import table to identify dependencies.
- Build a recursive dependency tree by opening and parsing each dependent DLL.
- Provide the dependency tree data to the frontend.

#### 3.3.2 Interfaces
- `build_dependency_tree(dll_file: &DLLFile) -> Result<DependencyTreeNode, Error>`
    - Constructs a dependency tree starting from the given DLL file.

- `resolve_dependencies(imports: &[ImportFunction]) -> Result<Vec<DLLFile>, Error>`
    - Resolves the dependencies by finding and parsing each imported DLL.

#### 3.3.3 Data Structures
- **DependencyTreeNode**
    - Represents a node in the dependency tree, with a reference to the DLL file and its dependencies.

#### 3.3.4 Error Handling
- If a dependency is missing or cannot be parsed, an `Error::MissingDependency` or `Error::ParseError` is returned.

### 3.4 Error Handling Module

#### 3.4.1 Responsibilities
- Capture and log errors encountered during file I/O, parsing, or dependency analysis.
- Provide error messages to the frontend for user display.

#### 3.4.2 Interfaces
- `handle_error(err: Error)`
    - Handles the given error by logging it and notifying the frontend.

#### 3.4.3 Error Types
- **Error::FileNotFound**
    - Indicates that a DLL file could not be found.

- **Error::ReadError**
    - Indicates that an error occurred while reading from a file.

- **Error::ParseError**
    - Indicates that a parsing error occurred, possibly due to an invalid or corrupted file.

- **Error::MissingDependency**
    - Indicates that a required dependency could not be found.

### 3.5 Data Structures

#### 3.5.1 DLLFile
- **Description:** Represents a DLL file and its parsed contents.
- **Fields:**
    - `path: String`: The file path of the DLL.
    - `pe_header: PEHeader`: The parsed PE header.
    - `imports: Vec<ImportFunction>`: The list of imported functions.
    - `exports: Vec<ExportFunction>`: The list of exported functions.
    - `dependencies: Vec<String>`: A list of DLLs that this DLL depends on.

#### 3.5.2 PEHeader
- **Description:** Represents the PE header in a DLL file.
- **Fields:**
    - `machine: u16`: The type of target machine.
    - `number_of_sections: u16`: The number of sections in the DLL.
    - `time_date_stamp: u32`: The time the file was created.
    - `size_of_optional_header: u16`: The size of the optional header.
    - `characteristics: u16`: Flags indicating attributes of the file.

#### 3.5.3 ImportFunction
- **Description:** Represents a function imported by the DLL.
- **Fields:**
    - `name: String`: The name of the function.
    - `ordinal: u16`: The ordinal number of the function.
    - `hint: u16`: A hint for the loader.

#### 3.5.4 ExportFunction
- **Description:** Represents a function exported by the DLL.
- **Fields:**
    - `name: String`: The name of the function.
    - `ordinal: u16`: The ordinal number of the function.
    - `address: u32`: The address of the function.

#### 3.5.5 DependencyTreeNode
- **Description:** Represents a node in the dependency tree.
- **Fields:**
    - `dll: DLLFile`: The DLL file represented by this node.
    - `dependencies: Vec<DependencyTreeNode>`: The list of dependencies.

## 4. Interaction Between Modules

### 4.1 Sequence Diagram

1. **File Selection:**
    - The frontend requests a file from the user.
    - The selected file path is sent to the File I/O Module.

2. **File Opening:**
    - The File I/O Module opens the file and passes the file handle to the PE Parser Module.

3. **PE Header Parsing:**
    - The PE Parser Module parses the PE header and returns the structured data.

4. **Import Table Parsing:**
    - The PE Parser Module extracts the Import Table and returns the list of imported functions.

5. **Dependency Resolution:**
    - The Dependency Analysis Module uses the list of imports to open and parse each dependent DLL, constructing a dependency tree.

6. **Error Handling:**
    - Any errors encountered during these steps are captured by the Error Handling Module and reported to the frontend.

## 5. Error Handling Strategy

### 5.1 Logging
Errors are logged with detailed information, including the type of error, the file or function involved, and a timestamp.

### 5.2 User Notification
Critical errors that prevent further analysis are reported to the user through the frontend, with an appropriate message.

### 5.3 Graceful Degradation
Where possible, non-critical errors (e.g., missing optional dependencies) are handled in a way that allows the application to continue functioning.

## 6. Performance Considerations

### 6.1 Concurrency
- The Rust backend will leverage Rust's concurrency features (e.g., threads, async/await) to perform file I/O and parsing operations in parallel, improving performance, especially when dealing with large numbers of dependencies.

### 6.2 Caching
- Frequently accessed DLLs and their parsed data can be cached in memory to reduce redundant I/O operations.

### 6.3 Lazy Loading
- Dependency trees can be constructed lazily, loading and parsing only the nodes required by the user's current view.

## 7. Security Considerations

### 7.1 Input Validation
- All input from the user (e.g., file paths) should be validated to prevent security vulnerabilities such as path traversal attacks.

### 7.2 Safe Memory Handling
- Rust’s ownership model will be leveraged to ensure memory safety, preventing common issues like buffer overflows and use-after-free errors.

### 7.3 Dependency Management
- Dependencies used in the project (both Rust crates and external libraries) will be regularly audited and updated to prevent vulnerabilities.


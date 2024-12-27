use prompt_macro::prompt;

prompt!(code_review_prompt,
r#"Project Path: {path}

I want you to carefully review the code in this project. Take your time, think step-by-step, and consider all the code paths and interactions between different parts of the codebase.

Source Tree: 
{tree}
{files}

Please analyze this codebase and provide:
1. A high-level overview of the project structure
2. Key components and their responsibilities
3. Any potential improvements or concerns
4. Code quality assessment
5. Suggestions for better organization or patterns"#);

prompt!(security_audit_prompt,
r#"Project Path: {path}

Please perform a security audit of this codebase:

Source Tree:
{tree}
{files}

Focus on:
1. Input validation and sanitization
2. File system operations security
3. Memory safety concerns
4. Error handling robustness
5. Potential security vulnerabilities"#);

prompt!(documentation_prompt,
r#"Project Path: {path}

Generate comprehensive documentation for this codebase:

Source Tree:
{tree}
{files}

Please provide:
1. Project overview and purpose
2. Installation and setup instructions
3. API documentation
4. Usage examples
5. Architecture explanation"#);
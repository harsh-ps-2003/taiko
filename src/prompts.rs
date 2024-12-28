use prompt_macro::prompt;

prompt!(code_review, r#"
# Code Review

{content}

Please perform a thorough code review of the above codebase, focusing on:
1. Code quality and best practices
2. Performance considerations
3. Error handling
4. Potential bugs or issues
5. Architecture and design patterns
6. Suggestions for improvements

Provide specific examples and recommendations where applicable.
"#);

prompt!(security_audit, r#"
# Security Audit

{content}

Please perform a comprehensive security audit of the above codebase, focusing on:
1. Common security vulnerabilities
2. Input validation and sanitization
3. Authentication and authorization
4. Data handling and privacy
5. Dependency security
6. Secure coding practices

Provide specific findings and recommendations for each identified issue.
"#);

prompt!(documentation, r#"
# Documentation Generation

{content}

Please generate comprehensive documentation for this codebase, including:
1. Project overview and purpose
2. Architecture and design decisions
3. Setup and installation instructions
4. API documentation (if applicable)
5. Usage examples
6. Contributing guidelines

Focus on clarity and completeness while maintaining technical accuracy.
"#);
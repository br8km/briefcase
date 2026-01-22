# Product Requirements Document: `briefcase`

---

## Overview

A CLI application to back up small size (0~10MB) personal sensitive data on Linux machine.


## Coding Principles

- Domain-Driven-Degisn
- Test-Driven-Development
- Modularity and Organization
- Data Integrity and Safety
- Robust Error Handling
- Minimize unsafe Code
- Detailed Logging


## Features


### Config Management

1. init with default config if not present, create config file if not exist
2. edit config variables
3. validate config variables


### Data Operation

1. Read config file for folder of Firefox profile
3. Export Firefox bookmarks and saved passwords into temp folder
2. Read config file for folder of personal sensitive data
4. Copy personal sensitive data into temp folder


### Encryption and Decryption

1. input password with/out hint
2. generate hash from password and store it into config file
3. validate input password with stored hash
4. encrypt|decrypt Firefox data and sensitive data


### Sync

1. sync data to remote cloud storage providers
2. sync data to remote sftp servers


## Naming and Locations

- The name of application: `<appname>` = `briefcase`
- The application will use **TOML** as config file format
- The config file location: `$Home/.config/<appname>/<appname>.toml`


## Non-Functional Requirements

### Performance
- **Load Time:** [Target load time]
- **Concurrent Users:** [Expected number]
- **Response Time:** [Target response time]

### Security
- **Authentication:** [Requirements]
- **Authorization:** [User permission levels]
- **Data Protection:** [Requirements]

### Compatibility
- **Devices:** [Supported devices]
- **Browsers:** [Supported browsers and versions]
- **Screen Sizes:** [Supported dimensions]

### Accessibility
- **Compliance Level:** [e.g., WCAG 2.1 AA]
- **Specific Requirements:** [Key accessibility features]

## Technical Specifications

### Frontend
- **Technology Stack:** [Framework, libraries]
- **Design System:** [Design system to use]
- **Responsive Design:** [Requirements]

### Backend
- **Technology Stack:** [Languages, frameworks]
- **API Requirements:** [RESTful, GraphQL, etc.]
- **Database:** [Database type and structure]

### Infrastructure
- **Hosting:** [Hosting solutions]
- **Scaling:** [Scaling requirements]
- **CI/CD:** [Deployment process]

## Analytics & Monitoring

- **Key Metrics:** [Metrics to track]
- **Events:** [User events to capture]
- **Dashboards:** [Required dashboards]
- **Alerting:** [Alert thresholds]

## Release Planning

### MVP (v1.0)
- **Features:** [List of MVP features]
- **Timeline:** [Expected release date]
- **Success Criteria:** [How to measure MVP success]

### Future Releases
- **v1.1:** [Feature set and expected timeline]
- **v1.2:** [Feature set and expected timeline]
- **v2.0:** [Feature set and expected timeline]

## Open Questions & Assumptions

- **Question 1:** [Open question]
- **Question 2:** [Open question]
- **Assumption 1:** [Assumption made]
- **Assumption 2:** [Assumption made]

## Appendix

### Competitive Analysis
- **Competitor 1:** [Strengths and weaknesses]
- **Competitor 2:** [Strengths and weaknesses]

### User Research Findings
- **Finding 1:** [Key insight from research]
- **Finding 2:** [Key insight from research]

### AI Conversation Insights
- **Conversation 1:** [Date, AI model used, key insights]
- **Conversation 2:** [Date, AI model used, key insights]
- **AI-Generated Edge Cases:** [List of scenarios the AI helped identify]
- **AI-Suggested Improvements:** [Major improvements suggested by AI]

### Glossary
- **Term 1:** [Definition]
- **Term 2:** [Definition]
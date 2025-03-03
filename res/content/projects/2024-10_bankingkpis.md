---
name: BankingKPIs
title: Data Extraction from PDF and XML
subtitle: Leveraging OCR, LLMs and parsers to extract KPIs from banking documents.
image: bankingkpis.webp
time: Oct, 2024 - present
skills: [python, streamlit, azure, postgresql, powerbi, langchain, git, githubactions]
url:
url_git:
---
*DISCLAIMER: This internal project is currently during my time at <a href="https://www.bearingpoint.com/en/?noRedirect=1" target="_blank">BearingPoint GmbH</a>. For compliance reasons, I can only briefly describe the contents of the project and my contribution to it.*

# Objectives & Implementation
In the past, data extraction processes were done manually by reading through each document and typing the value into an Excel document. The main objective of the project is to now **automatically extract relevant KPIs** (chosen by the department & user) in a **structured** manner from banking documents instead of the former manual processes, first and foremost PDFs. Banking statements are often hundreds of pages long and thus an automization is not trivial. We first developed a **multi-agent RAG retrieval** using <a href="https://www.langchain.com/langgraph" target="_blank">LangGraph</a> but quickly saw the limitations of current LLMs, when trying to extract one specific KPI on one specific page from a e.g. 500-page document.

Due to time pressure, we developed a semi-automatic approach but fully usable with our application, for the time being without GenAI. The application is developed in <a href="https://streamlit.io/" target="_blank">Streamlit</a> and CI/CD via <a href="https://learn.microsoft.com/en-us/azure/developer/github/github-actions" target="_blank">GitHub Actions to Azure</a>. We store the extracted KPIs in a <a href="https://www.postgresql.org/" target="_blank">PostgreSQL database</a> on Azure and also store the documents in an Azure Blob Container. For further (low-code) analysis we are setting up a <a href="https://www.microsoft.com/de-de/power-platform/products/power-bi" target="_blank">PowerBI</a> dashboard and the connection to the PostgreSQL database.

Over the coming months, we will perhaps explore silo process tasks of the project using LLMs, e.g. only extraction from one target page, not entire document. As we are currently storing valuable extraction information and creating textual splits using OCR, fine-tuning a LLM could also enhance our capabilities. Further, we are testing the extraction of KPIs by parsing a standardized reporting format for banks.

# My role
I am leading the technological development of the tool itself and architecture behind it. For me, this project is very challenging but rewarding with its steep learning curve. The project has become a **full-stack project**, with lots of backend endpoints and an in-production frontend application. Just some of the new domains I have learned are:
- **Building** a production-ready application, i.e. concurrency, authentication, bug & crash reports, ...
- **Deploying** a production-ready application, i.e. CI/CD with testing environment, cloud setup, scaling, data security, ...
- **Modeling** of multi-agent LLM systems and how they interact
- Better **understanding** of chosen technologies
- **Project management**, trying to keep communication with IT colleagues, department and users fast & direct to ensure quick & agile development
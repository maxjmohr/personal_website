---
name: CampaignManagement
title: Analytical Campaign Management
subtitle: Implementating and analyzing regular and ad-hoc marketing campaigns.
image: campaign_management.png
time: Jun, 2021 - Nov, 2023
skills: [sql, oracledb, excel, visualbasic, atlassian, git]
url:
url_git:
---
*DISCLAIMER: This project was during my time at <a href="https://www.bearingpoint.com/en/?noRedirect=1" target="_blank">BearingPoint GmbH</a> with an external client. For compliance reasons, I can only briefly describe the contents of the project and my contribution to it.*

# Objectives & Implementation
For an automotive captive, we assisted in the campaign and customer relationship management. The client had both **regular, e.g. monthly, and ad-hoc campaigns**. For all relevant campaigns, we selected the campaign participants with technical restraints, e.g. age, contract_since, as well as created reportings for multiple departments. For regular campaign selections and reports, we developed multiple **packages** in our <a href="https://www.oracle.com/database/" target="_blank">Oracle DB</a>. The packages contained computation logic for central KPIs as well as functions to generate the correct datasets for further processing. To execute these packages, we connected them with a central <a href="https://excel.cloud.microsoft/de-de/" target="_blank">Excel</a> file via **Visual Basic macros**. The user was then able to generate datasets and reports by the press of a button in the central Excel file. While some reports displayed the results and retention of the campaigns, others were analaysis on the existing financial contracts, e.g. regarding their expiration and renewals.

With the campaign management came the maintenance and optimization of a silo data warehouse and surrounding silos. During the lifespan of the project, many objects were simplified and merged to create a **much reduced, slim database model**. We also optimized many scripts for **deduplication and quicker runtime**. Version control was via <a href="https://bitbucket.org/product/" target="_blank">Atlassian Bitbucket</a>.

The project was managed via <a href="https://www.atlassian.com/software/jira" target="_blank">Atlassian Jira</a>. We documented all of the technical and technological knowledge together with the departments in a knowledge database (<a href="https://www.atlassian.com/software/confluence" target="_blank">Atlassian Confluence</a>).

# My role
For me personally, this was my introduction to IT consulting projects as it was **my first external project** since joing the firm. This meant that in the beginning, through **learning-on-the-job**, I got into topics such as database management and SQL, Excel macros, (agile) project management via Jira and so on. Initially, I created a lot of regular selections and reports, which were relatively stable, though bugs meant I was able to jump into the codebase, understand it and fix it. My mentors also taught me early on about naming and coding conventions and project communication. For every task and change in the database, we had **quality assurances** with the department, which every developer (including myself) conducted themselves.

With time, I took over more and **more complex tasks** and built ad-hoc selections from scratch, as well as built new reports and redesigned existing ones (also changing the underlying datasets by applying changes to the database packages). I also helped **onboard new colleagues** onto the project and supported them in their task executions.
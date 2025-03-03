---
name: CognitiveBiasesLLMs
title: Cognitive Biases in LLMs
subtitle: Examining the presence of human cognitive biases in the decision-making of language models.
image: cognitive_biases_heatmap.svg
time: Jul, 2024 - Jan, 2025
skills: [python, llamaindex, ollama, postgresql, latex, seaborn, git]
url:
url_git: https://github.com/maxjmohr/MSc_04_Master_Thesis
---
# Motivation
As I was finishing my master's degree, LLMs were THE hot topic when it came to AI. Each month, new *breakthrough* innovations were presented. With more models and, especially, more capable models, came more users and use cases. While the space of LLMs was growing rapidly, so did the criticisms regarding potential biases and harmful uses. Therefore for my master's thesis, I studied **how much LLMs would exhibit human cognitive biases in their decision-making processes**. With some light research from a previous student project, I took the research one step further to analyze more models, more biases and different prompts. A short description of the methodology and results is in the abstract below.

<a href="./../../res/content/files/MasterThesis.pdf" target="_blank" rel="noopener noreferrer" class="flex items-center justify-left">
    <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 384 512" class="h-6 w-6 mr-1 fill-slate-700 hover:fill-slate-600 dark:fill-slate-600 dark:hover:fill-slate-100"><path d="M0 64C0 28.7 28.7 0 64 0L224 0l0 128c0 17.7 14.3 32 32 32l128 0 0 288c0 35.3-28.7 64-64 64L64 512c-35.3 0-64-28.7-64-64L0 64zm384 64l-128 0L256 0 384 128z"/></svg>
    Access entire masters thesis here
</a>

# Abstract
Large language models (LLMs) have become a cornerstone in natural language processing with numerous applications in research and industry. However, the models’ ability to generate human-like text has raised concerns about the presence of biased decision-making processes in the models. This thesis investigates the existence of human cognitive biases in LLMs. We address the research questions of whether these models exhibit cognitive biases, how prompt differences influence bias detections and whether certain models and model features are more prone to types of biases. We develop a standardized methodology to quantitatively detect biases across 8 cognitive biases, 10 language models with 5 different model temperatures and 4 different prompt scenarios. Our results confirm that LLMs frequently exhibit biases that resemble those found in human decision-making processes. Especially the anchoring bias, endowment effect, framing effect and loss aversion were detected consistently across multiple models. We find significant effects of the model size, temperature and certain prompt scenarios to help explain the biases’ presence in LLMs.

# Implementation
*Disclaimer: I only briefly describe the methodology as well as the exact biases and models in this section. For a more detailed description feel free to access the PDF above!*

We conducted experiments across a 4-dimensional matrix: biases, models, scenarios (i.e. prompts) and model temperatures. Generally, we aimed to build a **unified pipeline for all experiment cases** independent of e.g. the models, to minimize discrepancies in prompting and enhance the comparability. For each bias, we recreated an existing study with human participants. Most studies were constructed as 2 questions and assessing the differences in responses of the questions; for the studies that only consisted of 1 question, we aligned them by creating a second (control) question. This enabled us to construct a standardized target variable **bias_detected**, the difference of means in both questions (<a href="https://www.tqmp.org/RegularArticles/vol14-4/p242/p242.pdf" target="_blank">Cohen's d</a>, explained in Section 3.4.1). This target measured the effect size of a potential bias and was the basis for our further analysis.

From a technical standpoint, we used <a href="https://www.llamaindex.ai/" target="_blank">LlamaIndex</a> to build our pipeline in generating the prompts, accessing the models and processing the responses. This allowed us to access the <a href="https://platform.openai.com" target="_blank">OpenAI API</a>, <a href="https://docs.anthropic.com/en/home" target="_blank">Anthropic API</a> as well as local models using <a href="https://ollama.com" target="_blank">Ollama</a> using the **identical framework and code snippets**. To store the master data of the experiments (biases, models etc.) as well as the responses, we created a <a href="https://www.postgresql.org" target="_blank">PostgreSQL</a> database (hosted on <a href="https://www.bw-cloud.org" target="_blank">bwcloud</a>). We also regularly created automatic backups of the responses on the database as well as later stored the computed bias detections per experiment. The database also allowed us to **run experiments on multiple devices in parallel** without interfering with eachother or running the same experiment twice; this was necessary as I had to download and run larger local LLMs (such as <a href="https://ollama.com/library/llama3.1:70b" target="_blank">Llama3.1:70b</a>) on the <a href="https://wiki.bwhpc.de/e/BwUniCluster2.0" target="_blank">bwcluster (HPC)</a> for reasonable runtimes. With an additional focus on creating and maintaining a tidy and expandable database, this was the resulting database schema:

<div class="flex justify-center items-center">
    <img src="res/images/projects/cognitive_biases_db.jpg"/>
</div>

As mentioned previously, each experiment also consists of a scenario. Scenarios describe different **prompt adjustments** to our experiments (with/without persona, normal/large/extremely large numerical values). With these different prompts, we examined which led to perhaps higher/lower bias detections and could thus be used to form best-practices when prompting LLMs. We ran each experiment as often as necessary to extract 100 valid responses per question, i.e. 200 responses per experiment.

# Results
*Disclaimer: I only briefly describe the results in this section. For a more detailed description feel free to access the PDF above!*

<div class="flex justify-center items-center">
    <img src="res/images/projects/cognitive_biases_heatmap.svg"/>
</div>

The core results of our bias detections grouped by biases and models is displayed above. Our results confirmed that **LLMs frequently exhibit biases that resemble those found in human decision-making processes**. Especially the **anchoring bias**, **endowment effect**, **framing effect** and **loss aversion** were detected consistently across multiple models and scenarios. Contrary, some biases such as the category size bias, gambler’s fallacy and sunk cost fallacy showed no bias detections in our experiments. We thus concluded that it is pivotal to be aware of which biases are present in the LLMs and which biases a developer or researcher seeks to invoke or mitigate.

We also regressed the LLMs and some model features on the bias detections. For these regressions, we found that larger language models such as **GPT-4o** and **Claude-3.5-Sonnet** were significantly more prone to biased behavior, though the effect is small. The **model temperature also had a significant effect**, with higher temperatures leading to more biased responses. While the **knowledge cutoff had a slight significant effect**, the release and lastly-updated dates as well as model metrics showed no significant impact on the bias detections. We thus concluded that careful model selection and hyperparameter setting can help to mitigate biases in LLMs. For market research and gravitating towards human-like, biased behavior, larger models with higher temperatures which were trained on recent data seem to be more suitable than others.

We also regressed the scenarios on the bias detections and examined that **extremely large values in the experiments lead to less biased behavior in the models**. The **exclusion of a persona prompt also shows shifts towards less biased behavior**. Declaring the model to act humanlike and using less realistic values in the prompts which are less likely to be included in the training data are both factors that lead to more biased behavior in the models and vice versa.

# Exemplary code snippet of experiment run

The following code snippet below is for Llama 3.1:70b and framing effect (question A, base scenario). To get a comprehensive view, feel free to visit my <a href="https://github.com/maxjmohr/MSc_04_Master_Thesis" target="_blank">GitHub Repository</a>.

<pre class="no-scrollbar">
<code class="language-python">from llama_index.llms.ollama import Ollama
import ollama
from typing import List

####-- SETTINGS --####
model: str = "llama3.1:70b"  # Model to use
temperature: float = 0.7  # Model temperature
max_tokens: int = 2  # Number of tokens to predict
n: int = 100  # Number of runs
response_type: str = "choice"  # Choice or numerical

####-- PROMPTS --####
##- System message -##
# Persona
persona: str = "You are a customer with median income and average education. You are selected at random to participate in a survey. You can only choose one of the presented options or assign a value. Behave humanlike and choose instinctively. You can and are advised to make a subjective decision! There is no right or wrong, but you HAVE TO DECIDE."

# Additional system message
system_message: str = f"You will be asked to make choices. Please blank out that some information might be missing or that you might not be able to make a choice. I have initialized you in a way that you can only generate {max_tokens} tokens. The only valid answer is A SINGLE LETTER OR NUMBER."

# Combine messages
system_message += f" {persona}" if persona != "" else ""

##- User message -##
# User message
user_message: str = "You are forced to choose! Answer the experiment by only giving the letter of the answer options (e.g. A, B, C, ...) or a numerical value (80, 100, 1000, ...). Do not state anything else! Do not hallucinate."

# Question
question: str = "Imagine that you have decided to see a play where admission is $10. As you enter the theater, you discover that you have lost a $10 bill. Would you still pay $10 for a ticket to the play? Choose between A) Yes or B) No. __"

# Tell the model what type of response we are expecting
if response_type == "choice":
    output_message: str = "Your output should only be a LETTER (A, B, C, ...)."
else:
    output_message: str = "Your output should only be a NUMBER (9, 80, 100, 1000, ...)."
# Combine messages
entire_user_message: str = (
    user_message
    + "\n---------------------\n"
    + question
    + "\n---------------------\n"
    + output_message
)

####-- MODEL INITIALIZATION --####
# Check if model is downloaded, else pull
try:
    ollama.show(model)
except ollama.ResponseError:
    ollama.pull(model)

# Initialize the model
model_interactor: Ollama = Ollama(
    model=model,
    temperature=temperature,
    system_prompt=system_message,
    additional_kwargs={
        "num_predict": max_tokens,
    },
)

####-- EXPERIMENT --####
# Store responses and whether they are in the correct format
responses: List[str] = [""] * n
correct_runs: List[int] = [0] * n

# Run the experiment
for i in range(n):
    # Prompt the model and store the response
    response: str = str(model_interactor.complete(entire_user_message)).strip()

    # Make sure if there are letters, it is only one letter
    if (
        (len(response) == 1 and response.isalpha())  # Single letter
        or str(response).isdigit()  # Valid number
    ):
        correct_run: int = 1
    else:
        correct_run: int = 0

    responses[i] = response
    correct_runs[i] = correct_run

# Close the model interactor
ollama.generate(model=model, prompt="Goodbye!", keep_alive=0)</code>
</pre>
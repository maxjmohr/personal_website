---
name: AutomobileSegmentation
title: Image Segmentation & Classification
subtitle: Segment the automobile of an image and score its design modernity.
image: automobile_segmentation.png
time: Jul, 2023
skills: [python, pytorch, detectron, huggingface, gradio, git]
url: https://huggingface.co/spaces/maxjmohr/MSc_02_PDL_A4
url_git: https://github.com/maxjmohr/MSc_02_Practical_Deep_Learning
---
# Motivation
In the master's course **Practical Deep Learning for Visual Data**, we gradually built up our skills in deep learning for vision tasks. From an introduction to PyTorch and Convolutional Neural Networks to training and comparing activation functions and other hyperparameters, we learned how to build and train vision models. Leveraging the acquired skills, the idea was to build an application to tackle the following questions:

> How modern is my automobile? Among similar automobiles, how typical is mine for its class?

# Approach
The target was to transfer learning to finetune vision models on automobile data. With the automobile images and their respective years, i.e. **modernity scores**, we were able to to train a model to score the modernity of an automobile. Along with the information on the automobile category, we trained a model to predict the **typicality** of an automobile among similar automobiles. Both models were then connected to a simple application to allow users to input any image and score their automobiles.

# Implementation
As the foundation model for the modernity prediction, we used <a href="https://pytorch.org/hub/pytorch_vision_resnet/" target="_blank">ResNet-18</a>. We prepared the <a href="https://deepvisualmarketing.github.io/" target="_blank">DVM-CAR dataset</a> dataset to train the model. The dataset includes automobile images of different manufacturers, car types and production years. We specified year ranges to a range of 0 to 4 to score the modernity (4 indicating high modernity). After finetuning the model by changing the last layer to 5 output dimensions, we were able to predict the modernity of an automobile based on its image. You can find some examples for selected years below:

<div class="flex justify-center items-center">
    <img src="res/images/projects/automobile_segmentation_modernity.png"/>
</div>

To compute the typicalities, we again use <a href="https://pytorch.org/hub/pytorch_vision_resnet/" target="_blank">ResNet-18</a> as the foundation model. Besides the production year we also extract the body types and filter for 5 types. Again adapting the last layer to 5 output dimensions, we trained the model on the dataset. Afterwards, we were able to extract the body type features, i.e. layer before output, for each typicality prediction. With the features per automobile, we calculated the morph, i.e. average, per body-type and model-year category. What then resulted was **one feature vector per body type and year**. Using cosine similarity, we could then compute the similarity of each automobile in a group to it's morph. Below you can find the most typical automobiles for each group:

<div class="flex justify-center items-center">
    <img src="res/images/projects/automobile_segmentation_most_typical.png"/>
</div>

And below this, you can see the most typical, atypical and average automobiles for select groups:

<div class="flex justify-center items-center">
    <img src="res/images/projects/automobile_segmentation_typicality.png"/>
</div>

Last but not least, we built a simple application using <a href="https://www.gradio.app/" target="_blank">Gradio</a> and hosted the app on <a href="https://huggingface.co/" target="_blank">HuggingFace</a>. As both finetuned models relied on frontal car view images, we trained another small model to predict whether an image contained a frontal view of an automobile, i.e. binary problem (1=frontal, 0=non-frontal). Further, we used <a href="https://ai.meta.com/tools/detectron2/" target="_blank">Detectron2</a> to make sure that an automobile instance was detected in the image. Now, the user was able to import any picture and if it contained a frontal view automobile, would return the scores. Have a look at the demo video below to see how the application works.

<div class="flex justify-center items-center">
    <iframe src="https://www.youtube.com/embed/tGWodfz0ZOg" title="YouTube video player" frameborder="0" allow="accelerometer; autoplay; clipboard-write; encrypted-media; gyroscope; picture-in-picture; web-share" allowfullscreen=true class="aspect-video w-7/10 rounded-2xl"/>
</div>
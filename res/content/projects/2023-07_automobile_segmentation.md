---
name: AutomobileSegmentation
title: Image Segmentation & Classification
subtitle: Segment the automobile of an image and score its design modernity.
image: automobile_segmentation_cover.png
time: Jul, 2023
skills: [python, detectron, gradio, huggingface, pytorch, git]
url: https://huggingface.co/spaces/maxjmohr/MSc_02_PDL_A4
url_git: https://github.com/maxjmohr/MSc_02_Practical_Deep_Learning
coauthors:
report:
---
<div class="flex flex-row">
<div class="flex flex-col h-full mr-10">
    <!-- Idea -->
    <div class="shrink-0 lg:w-[30rem] bg-white dark:bg-gray-800 rounded-2xl mb-10">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-20 lg:h-20 fill-gray-700 dark:fill-stone-300 pt-4 pl-8">
            <path d="M19 6.734c0 4.672-4.25 7.079-4.25 12.266h-5.5c0-5.187-4.25-7.594-4.25-12.266 0-4.343 3.498-6.734 6.996-6.734 3.502 0 7.004 2.394 7.004 6.734zm-4.75 13.266h-4.5c-.276 0-.5.224-.5.5s.224.5.5.5h4.5c.276 0 .5-.224.5-.5s-.224-.5-.5-.5zm.25 2h-5l1.451 1.659c.19.216.464.341.753.341h.593c.288 0 .563-.125.752-.341l1.451-1.659z"/>
        </svg>
        <div class="py-8 pl-8 pr-8 lg:py-6 lg:pl-10 lg:pr-10">
            <p class="block text-left antialiased font-extrabold text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-6xl lg:text-4xl leading-[4.2rem]">Idea</p>
            <p class="block text-left antialiased font-medium text-stone-600 dark:text-neutral-300 text-6xl lg:text-lg py-2">The idea of the later assignments of the course was to be able to check the modernity and typicality of any automobile in an application.</p>
        </div>
    </div>
    <!-- Approach -->
    <div class="shrink-0 lg:w-[30rem] bg-white dark:bg-gray-800 rounded-2xl">
        <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-20 lg:h-20 fill-gray-700 dark:fill-stone-300 pt-4 pl-8">
            <path d="M6.57 13.41c-.373 0-.741-.066-1.093-.195l.407-1.105c.221.081.451.122.686.122.26 0 .514-.05.754-.148l.447 1.09c-.382.157-.786.236-1.201.236zm8.67-.783l-1.659-.945.583-1.024 1.66.945-.584 1.024zm-6.455-.02l-.605-1.011 1.639-.981.605 1.011-1.639.981zm3.918-1.408c-.243-.101-.5-.153-.764-.153-.23 0-.457.04-.674.119l-.401-1.108c.346-.125.708-.188 1.075-.189.42 0 .83.082 1.217.244l-.453 1.087zm-8.734-.163c-.535 0-.969.433-.969.968 0 .535.434.968.969.968.535 0 .969-.434.969-.968-.001-.535-.434-.968-.969-.968zm13.576-7.036l-5.545-4-5.545 4-6.455-4v20l6.455 4 5.545-4 5.545 4 6.455-4v-20l-6.455 4zm4.455 14.887l-4 2.479v-4.366h-1v4.141l-4-2.885v-4.256h-2v4.255l-4 2.885v-5.14h-1v5.365l-4-2.479v-15.294l4 2.479v2.929h1v-2.927l4-2.886v3.813h2v-3.813l4 2.886v1.927h1v-1.929l4-2.479v15.295zm-1.328-4.871l-1.296-1.274 1.273-1.293-.708-.702-1.272 1.295-1.294-1.272-.703.702 1.296 1.276-1.273 1.296.703.703 1.277-1.298 1.295 1.275.702-.708z"/>
        </svg>
        <div class="py-8 pl-8 pr-8 lg:py-6 lg:pl-10 lg:pr-10">
            <p class="block text-left antialiased font-extrabold text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-6xl lg:text-4xl leading-[4.2rem]">Approach</p>
            <p class="block text-left antialiased font-medium text-stone-600 dark:text-neutral-300 text-6xl lg:text-lg py-2">The approach was to train vision models on the automotive design modernity as well as the typicality across other similar automobiles. Further, we trained a model to detect whether we input the frontal view of an automobile as this affects the other models. With these models and a suitable application, the user should be able to import any image with a frontal view image of a car and score the modernity and typicality.</p>
        </div>
    </div>
</div>
<!-- Implementation -->
<div class="shrink-0 w-[57rem] h-fit mr-10 bg-white dark:bg-gray-800 rounded-2xl mb-10">
    <svg xmlns="http://www.w3.org/2000/svg" width="24" height="24" viewBox="0 0 24 24" class="w-16 h-16 lg:w-20 lg:h-20 fill-gray-700 dark:fill-stone-300 pt-4 pl-8">
        <path d="M8.566 17.842c-.945 2.462-3.678 4.012-6.563 4.161.139-2.772 1.684-5.608 4.209-6.563l.51.521c-1.534 1.523-2.061 2.765-2.144 3.461.704-.085 2.006-.608 3.483-2.096l.505.516zm-1.136-11.342c-1.778-.01-4.062.911-5.766 2.614-.65.649-1.222 1.408-1.664 2.258 1.538-1.163 3.228-1.485 5.147-.408.566-1.494 1.32-3.014 2.283-4.464zm5.204 17.5c.852-.44 1.61-1.013 2.261-1.664 1.708-1.706 2.622-4.001 2.604-5.782-1.575 1.03-3.125 1.772-4.466 2.296 1.077 1.92.764 3.614-.399 5.15zm11.312-23.956c-.428-.03-.848-.044-1.261-.044-9.338 0-14.465 7.426-16.101 13.009l4.428 4.428c5.78-1.855 12.988-6.777 12.988-15.993v-.059c-.002-.437-.019-.884-.054-1.341zm-5.946 7.956c-1.105 0-2-.895-2-2s.895-2 2-2 2 .895 2 2-.895 2-2 2z"/>
    </svg>
    <div class="py-8 pl-8 pr-8 lg:py-6 lg:pl-10 lg:pr-10">
        <p class="block text-left antialiased font-extrabold text-gray-700 dark:text-stone-200 text-opacity-90 dark:text-opacity-90 text-6xl lg:text-4xl leading-[4.2rem]">Implementation</p>
        <div class="flex justify-center items-center pt-4 pb-4">
            <img class="object-scale-down lg:max-h-96 px-4 rounded-xl" src="res/images/projects/automobile_segmentation_modernity.png"/>
            <img class="object-scale-down lg:max-h-96 px-4 rounded-xl" src="res/images/projects/automobile_segmentation_typicality.png"/>
        </div>
        <div class="flex flex-row">
            <p class="block text-left antialiased font-medium text-stone-600 dark:text-neutral-300 text-6xl lg:text-lg py-2 pr-4">Firstly, we had to train vision models on automobile data to score the modernity and typicality. We used the 
            <a href="https://deepvisualmarketing.github.io/" target="_blank" class="italic lg:hover:font-bold">DVM-CAR dataset</a>
             to get frontal view images of various automobiles. Both models were based on  
            <a href="https://pytorch.org/hub/pytorch_vision_resnet/" target="_blank" class="italic lg:hover:font-bold">ResNet-18</a>
             and were finetuned with the data. For the modernity score, we specified year ranges to a raange of 0 to 4. For the typicality score, we extracted the features (model types) of the finetuned model. For each group, we the computed the morph per body-type and model-year category. Then we calculated the cosine similarity of each observation to the group's morph. 
            </p>
            <p class="block text-left antialiased font-medium text-stone-600 dark:text-neutral-300 text-6xl lg:text-lg py-2">To make sure we input the data in the correct manner, we used Detectron2 by Meta to detect the automobile instance in an image and trained a simple neural network to distinguish between frontal and alternative view images. We built a simple application using Gradio and hosted the app on Huggingface. Now, the user was able to import any picture and if it contained a frontal view automobile, would return the scores.</p>
        </div>
    </div>
</div>
</div>
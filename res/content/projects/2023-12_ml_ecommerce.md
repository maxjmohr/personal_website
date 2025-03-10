---
name: MLeCommerce
title: Machine Learning Mini Project
subtitle: Enhancing voucher assignment for target customers using machine learning.
image: ml_ecommerce.webp
time: Dec, 2023 - Jan, 2024
skills: [python, scikitlearn, pytorch, git]
url:
url_git: https://github.com/maxjmohr/MSc_03_ML_Applications_MiniProject
---
# Motivation
As part of the course 'Machine Learning Applications' during my master's, students were asked to compete against eachother in groups of two to build the best model for a voucher assignment. The task was fairly simple:

> Empirical analyses conducted by the media retailer have demonstrated that for 10% of non-buyers, the voucher instigates a purchase with an average order value of €20. Thus, if a voucher is dispatched to a customer who would not have actually made another purchase, the revenue increases by an average of €1.5. On the other hand, sending a voucher to a customer who would have made a purchase anyway results in a revenue loss equivalent to the voucher value of €5. For customers who don’t receive a voucher, there is no impact on revenues. The task at hand involves constructing a predictive model that leverages various features associated with a customer’s initial order. The objective is to **determine whether a €5.00 voucher should be issued to a specific customer.** The model should be designed to predict if a customer will place a subsequent order within a 90-day period following their initial purchase.

We were handed a training dataset along with a datasheet as well as a test dataset (which was in the same style as the one used by the professor for benchmarking the models).

# My role
From a project management standpoint, both of us students were in charge to manage the project and we both collectively prepared the data and developed the models. The project gave me the chance to work on the entire pipeline from data preparation, feature analysis, training und tuning models as well as presenting the results in a business context.

# Approach & Implementation

Our approach was quite clear: before modelling, we needed to **prepare and clean the dataset along with some feature engineering**. With some of the date columns, we engineered features representing the difference of some dates, e.g. if there was a delivery delay. We simplified some columns, e.g. if they used an advertising code == 1, else 0, and aggreageted some, e.g. total number of books ordered. As we found an imbalance of datasets with our target variable (reordered after 90 days) of 1 to much more data entries with no reorder, we used **SMOTE** to oversample the minotry class (reorder == 1). To encode categorical features we used the OrdinalEncoder and scaled numerical features using MinMaxScaler. Afterwards, we had a balanced, scaled and encoded dataset with some more perhaps interesting features.

For our model comparisons, we trained the following models: <a href="https://scikit-learn.org/stable/modules/generated/sklearn.ensemble.RandomForestClassifier.html" target="_blank">Random Forest</a>, <a href="https://scikit-learn.org/stable/modules/generated/sklearn.ensemble.AdaBoostClassifier.html" target="_blank">AdaBoost</a>, <a href="https://xgboost.readthedocs.io/en/stable/" target="_blank">XGBoost</a>, <a href="https://scikit-learn.org/stable/modules/generated/sklearn.svm.SVC.html" target="_blank">SVM</a> and <a href="https://pytorch.org/tutorials/beginner/basics/buildmodel_tutorial.html" target="_blank">Multi-Layer Perceptron Neural Networks</a>, all with hyperparameter tuning. We optimized each model for the **loss function which we constructed from the revenue hints** in the task description. For each model, we used **feature importances to define the top 2/3 of features** used in the model to retrain it, only using these features. Lastly, we used the best 3 models to form an **ensemble model** which turned out to perform the best (with the top 2/3 of features):

<div class="flex justify-center items-center">
    <img src="res/images/projects/ml_ecommerce_model_results.png"/>
</div>

With the predictive ensemble model, we were able to achieve a 25% higher revenue on the test dataset, sharing this top benchmarking results with other groups.

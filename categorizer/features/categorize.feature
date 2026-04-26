Feature: Message can be categorized

Scenario: Categorize by exact regex match
Given a message with content "i want to know more about corn"
    And a "regex" category 
    And expression "(?im)\b(corn)\b" 
    And confidence "1.0"
When I categorize the message
Then I should see the category match as "yes"
    And confidence "1.0"

Scenario: Categorize by partial regex match - fat fingered word
Given a message with content "i want to know more about cron"
    And a "regex" category 
    And expression "(?ims)\b(cron|crn|con|ron)\b" 
    And confidence "0.66"
When I categorize the message
Then I should see the category match as "yes"
    And confidence "0.66"

Scenario: Categorize by partial regex match - joined word
Given a message with content "i want to know more about cron"
    And a "regex" category 
    And expression "(?im)\b(corn|cron|crn|con|ron)[\w\d]+\b" 
    And confidence "0.33"
When I categorize the message
Then I should see the category match as "yes"
    And confidence "0.33"

Scenario: Categorize by cosine similarity
Given a message with content "i want to know more about corn please"
    And a "cosine" category 
    And model "nomic-embed-text" 
    And cosine for text "corn"
    And min confidence "0.5"
    And max confidence "0.75"
When I categorize the message
Then I should see the category match as "yes"
    And min confidence "0.50"
    And max configence "0.75"

Scenario: Categorize by llm
Given a message with content "i want to know more about corn please"
    And a "llm" category 
    And model "llama3.2:1.5B"
    And prompt "is the message ####I want to know more about corn please### talking about ||||corn||||?  Respond Yes or No in json format and provide your confidence level."
When I categorize the message
Then I should see the category match as "yes"
    And confidence "_"

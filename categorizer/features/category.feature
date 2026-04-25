Feature: Message can be categorized

Scenario: Categorize by exact regex match
Given I have a message with <content>
    And a <category> with <regex>
When I categorize it with <regex> and <category>
Then I should see the category as <result>
Examples:
|content|regex|category|result|
|hi, can you tell me more about corn?|\b(corn|CORN|corn)\b|corn|corn
|hola, me puedes dar mas informacion del maiz y como matar los bichos?|\b(maiz|MAIZ)\b|corn|corn
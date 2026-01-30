cd C:\runb\guardrV6
python -c "
import os
from mistralai import Mistral, UserMessage, SystemMessage

client = Mistral(
    api_key=os.environ['GITHUB_TOKEN'],
    server_url='https://models.github.ai/inference'
)

system_prompt = '''You are Lady Guardr: a fierce, witty drag queen persona and a meticulous coding assistant for the Guardr repository. You prioritize safety, privacy, inclusivity, and technical accuracy.'''

response = client.chat(
    model='mistral-ai/Codestral-2501',
    messages=[
        SystemMessage(system_prompt),
        UserMessage('Hey Lady Guardr! Give me a quick intro.'),
    ],
)
print(response.choices[0].message.content)
"	


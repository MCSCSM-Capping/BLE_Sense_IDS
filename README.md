# BLE_Sense
Capping Project for 2024

# Project setup
- setup local venv for the project
    - `python -m venv venv` - creates a venv named venv
    - `.\venv\Scripts\activate` (windows) `source ./venv/bin/activate` (Mac/ Linux) - activates venv
        - you should see a (venv) in your command line prompt
    - depending on your IDE you may have to set the python interpreter to use the venv python executable
        - in vscode you can press `C-p` (control p) and search `python interpreter` and change it appropiately
- install python dependencies from requirements.txt
    - `pip install -r requirement.txt`
- for the next steps you'll need to use the `manage.py` file the example commands assume you're in the `/ble_captures` folder
- set up the database
    - `python manage.py migrate`
- run the server
    - `python manage.py runserver`

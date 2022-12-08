import requests
from flask import Flask, g as app_ctx
import time
from copy import deepcopy
from datetime import datetime, timedelta

# ------------------------------------------------------------------------------
# INITIALISE
# ------------------------------------------------------------------------------


app = Flask(__name__)

ENDPOINT = "https://code-challenge-a.wellcome-data.org/api?limit=-1"
AUTH_TOKEN = "Bearer aGV5bm9sb29raW5naW5oZXJldGhpc2lzc2VjcmV0c2hoaGg="


@app.before_request
def logging_before():
    # Store the start time for the request
    app_ctx.start_time = time.perf_counter()


@app.after_request
def logging_after(response):
    # Get total time in milliseconds
    total_time = time.perf_counter() - app_ctx.start_time
    time_in_ms = int(total_time * 1000)
    # Log the time taken for the endpoint
    print(f'Response took: {time_in_ms} ms')
    return response

# ------------------------------------------------------------------------------
# ENDPOINT
# ------------------------------------------------------------------------------


@app.route("/")
def sumulative_report():
    endpoint = ENDPOINT
    headers = {"Authorization": AUTH_TOKEN}

    result = requests.get(endpoint, headers=headers).json()

    applications = result['items']
    total_applications = result['available_records']

    report = create_sumulative_report(applications, total_applications)

    return report

# ------------------------------------------------------------------------------
# CREATE SUMULATIVE REPORT
# ------------------------------------------------------------------------------


def create_sumulative_report(applications, total_applications):

    # PREPARE DATA -------------------------------------------------------------

    months = ["01", "02", "03", "04", "05",
              "06", "07", "08", "09", "10", "11", "12"]

    research_areas = []
    mean_response_time_count = 0

    # finding out research areas could also happen in the main loop - but cost of running loop twice is negligible compared to API IO cost
    for item in applications:
        research_areas.append(item['research_area'])

    research_areas = list(set(research_areas))

    # For date debugging
    latest_date = datetime.strptime("1950-01-01", "%Y-%m-%d")
    earliest_date = datetime.strptime("2050-01-01", "%Y-%m-%d")

    research_area_summary = {
        'approved': 0,
        'submitted': 0,
        'rejected': 0
    }

    month_summary = {
        'approved': 0,
        'submitted': 0,
        'rejected': 0,
        'total_funding_granted': 0
    }

    output = {
        'totals_by_research_area': {research_area: deepcopy(research_area_summary) for research_area in research_areas},
        'totals_by_month_ytd': {month: deepcopy(month_summary) for month in months},
        'mean_response_time_days': None,
        'overdue_unactioned_application_ids': []
    }

    # set date to 2021-12-31 to make API run smoother // note to wellcome, update API data?
    # now = datetime.now()
    now = datetime.strptime("2021-12-31", "%Y-%m-%d")
    print(f'Current date set to: {now}')

    # CREATE OUTPUT OBJECT -----------------------------------------------------

    for item in applications:

        # 1. The total number of submitted, approved and rejected applications per research area
        output['totals_by_research_area'][item['research_area']
                                          ][item['status']] += 1

        # 2. For each of the past 12 months, the total submitted, approved, and rejected applications in each month
        # 3. For each of the past 12 months, the sum of funding we approved in each month based on the applications data.
        submitted_date = datetime.strptime(item['submitted_date'], "%Y-%m-%d")

        # this isn't perfect, i.e. leap years
        one_year_ago = now - timedelta(days=365)

        if submitted_date > one_year_ago:
            month_str = submitted_date.strftime("%m")

            # 2.
            output['totals_by_month_ytd'][month_str][item['status']] += 1

            # 3.
            if item['status'] == 'approved':
                output['totals_by_month_ytd'][month_str]['total_funding_granted'] += item['amount_awarded']

        # 4. The average time in (days) between an application being received (submitted) and an outcome (approved or rejected)
        # 5. A list of application ids which have not been actioned in more than 60 days from their submitted date (i.e. they are still in the submitted state).
        try:
            # 4.
            actioned_date = datetime.strptime(
                item['actioned_date'], "%Y-%m-%d")

            delta = (actioned_date - submitted_date).days
            mean_response_time_count += delta

        except:
            # 5.
            sixty_days_ago = now - timedelta(days=60)
            if submitted_date < sixty_days_ago:
                output['overdue_unactioned_application_ids'].append(
                    item['application_id'])

            # added debug code to find latest and earliest applications
            if latest_date < submitted_date:
                latest_date = submitted_date

            if earliest_date > submitted_date:
                earliest_date = submitted_date

    # 4.
    mean_response_time_days = mean_response_time_count / total_applications
    output['mean_response_time_days'] = int(mean_response_time_days)

    # print debug
    print(f'Oldest application is: {earliest_date}')
    print(f'Latest application is: {latest_date}')

    return output


# ------------------------------------------------------------------------------
# RUN APP
# ------------------------------------------------------------------------------
if __name__ == '__main__':
    app.run(debug=True)

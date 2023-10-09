document.addEventListener("DOMContentLoaded", function() {
    document.getElementById('fetchButton').addEventListener('click', fetchData);
});

async function fetchData() {
    try {
        const apiUrl = 'http://localhost:8000/activities';
        const response = await fetch(apiUrl);
        const data = await response.json();
        displayData(data);
    } catch (error) {
        console.error('Error fetching data: ', error);
        alert('Failed to fetch data. Check console for error details.');
    }
}

/*function displayData(data) {
    const dataContainer = document.getElementById('dataContainer');
    dataContainer.innerHTML = '';

    const dateLabel = document.createElement('p');
    dateLabel.textContent = `Date: ${data[0].date}`;
    dataContainer.appendChild(dateLabel);

    const table = document.createElement('table');
    dataContainer.appendChild(table);

    const headers = ['Name + Centre Name', 'Starting Time'];
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    headers.forEach(header => {
        const th = document.createElement('th');
        th.textContent = header;
        headerRow.appendChild(th);
    });
    thead.appendChild(headerRow);
    table.appendChild(thead);

    if (Array.isArray(data) && data.length > 0) {
        const tbody = document.createElement('tbody');
        table.appendChild(tbody);

        let groupedActivities = {};

        data.forEach(activity => {
            const key = `${activity.name} (${activity.centerName})`;

            if (!groupedActivities[key]) {
                groupedActivities[key] = [];
            }
            groupedActivities[key].push(activity.startTime);
        });

        Object.entries(groupedActivities).forEach(([nameCenter, times]) => {
            const row = document.createElement('tr');

            const nameTd = document.createElement('td');
            nameTd.textContent = nameCenter;
            row.appendChild(nameTd);

            times.forEach(time => {
                const timeTd = document.createElement('td');
                timeTd.textContent = time;
                row.appendChild(timeTd);
            });

            tbody.appendChild(row);
        });
    } else {
        dataContainer.innerHTML = '<p>No activities available.</p>';
    }
}*/

function displayData(data) {
    const dataContainer = document.getElementById('dataContainer');
    dataContainer.innerHTML = '';

    function formatDate(date) {
        const day = date.toLocaleString('default', { day: '2-digit' });
        const month = date.toLocaleString('default', { month: 'short' });
        return `${day} ${month}`;
    }

    const today = new Date();
    const tomorrow = new Date(today);
    tomorrow.setDate(today.getDate() + 1);
    const dayAfterTomorrow = new Date(today);
    dayAfterTomorrow.setDate(today.getDate() + 2);

        const dateBoxes = [
            { date: today, label: 'Today',id: 'todayBtn'  },
            { date: tomorrow, label: 'Tomorrow',id: 'tomorrowBtn' },
            { date: dayAfterTomorrow, label: 'Day after Tomorrow',id: 'dayAfterTomorrowBtn'  },
    ];

    const dateBoxContainer = document.createElement('div');
        dateBoxContainer.className = 'date-box-container';

    dateBoxes.forEach(({ date, label, id }) => {
        const dateBox = document.createElement('button');
        dateBox.className = 'date-box';
        dateBox.id = id;
        dateBox.textContent = `${formatDate(date)}`;
        dataContainer.appendChild(dateBox);
    });
    dataContainer.appendChild(dateBoxContainer);

    const table = document.createElement('table');
    dataContainer.appendChild(table);

    const headers = ['Name + Centre Name', 'Starting Time'];
    const thead = document.createElement('thead');
    const headerRow = document.createElement('tr');
    headers.forEach(header => {
        const th = document.createElement('th');
        th.textContent = header;
        headerRow.appendChild(th);
    });
    thead.appendChild(headerRow);
    table.appendChild(thead);

    if (Array.isArray(data) && data.length > 0) {
        const tbody = document.createElement('tbody');
        table.appendChild(tbody);

        let groupedActivities = {};

        data.forEach(activity => {
            const key = `${activity.name} (${activity.centerName})`;

            if (!groupedActivities[key]) {
                groupedActivities[key] = [];
            }
            groupedActivities[key].push(activity.startTime);
        });

        Object.entries(groupedActivities).forEach(([nameCenter, times]) => {
            const row = document.createElement('tr');

            const nameTd = document.createElement('td');
            nameTd.textContent = nameCenter;
            row.appendChild(nameTd);

            const timesContainer = document.createElement('div');
            timesContainer.className = 'times-container';

            times.forEach(time => {
                const timeBox = document.createElement('div');
                timeBox.className = 'time-box';
                timeBox.textContent = time;
                timesContainer.appendChild(timeBox);
            });

            const timeTd = document.createElement('td');
            timeTd.appendChild(timesContainer);
            row.appendChild(timeTd);

            tbody.appendChild(row);
        });

    } else {
        dataContainer.innerHTML = '<p>No activities available.</p>';
    }
}


function bookActivity(activityId) {
    alert(`Booking activity with ID: ${activityId}`);
}

function showInfoPopup(description) {
    const overlay = document.createElement('div');
    overlay.className = 'overlay';

    const modal = document.createElement('div');
    modal.className = 'modal';

    const closeButton = document.createElement('button');
    closeButton.textContent = 'Close';
    closeButton.onclick = () => document.body.removeChild(overlay);

    const descriptionP = document.createElement('p');
    descriptionP.textContent = description;

    modal.appendChild(descriptionP);
    modal.appendChild(closeButton);
    overlay.appendChild(modal);
    document.body.appendChild(overlay);
}

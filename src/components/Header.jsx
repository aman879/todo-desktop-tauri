import React, { useEffect, useState } from "react";

function Header({ route, onDateChange }) {
    const [dates, setDates] = useState({
        day: new Date(),
        week: new Date(),
        month: new Date(),
        year: new Date(),
    });

    useEffect(() => {
        if (route === "day") {
            onDateChange(formatDate(dates.day))
        } else if (route === "week") {
            onDateChange(getWeekRange(dates.week))
        } else if (route === "month") {
            onDateChange(dates.month.toLocaleDateString("en-US", { month: "short", year: "numeric" }))
        } else if (route === "year") {
            onDateChange(dates.year.getFullYear().toString())
        }
    }, [dates, route])

    const formatDate = (date) => {
        return date.toLocaleDateString("en-US", { month: "short", day: "numeric", year: "numeric" });
    };

    const formatDay = (date) => {
        return date.toLocaleDateString("en-US", { weekday: "long" });
    };

    const getWeekRange = (date) => {
        const start = new Date(date);
        start.setDate(start.getDate() - start.getDay()); // Get Sunday

        const end = new Date(start);
        end.setDate(end.getDate() + 6); // Get Saturday

        return `${formatDate(start)} - ${formatDate(end)}`;
    };

    const changeDate = (value) => {
        setDates((prev) => {
            const newDates = { ...prev };

            if (route === "day") {
                newDates.day = new Date(prev.day); 
                newDates.day.setDate(prev.day.getDate() + value);
            }
            if (route === "week") {
                newDates.week = new Date(prev.week);
                newDates.week.setDate(prev.week.getDate() + 7 * value);
            }
            if (route === "month") {
                newDates.month = new Date(prev.month);
                newDates.month.setMonth(prev.month.getMonth() + value);
            }
            if (route === "year") {
                newDates.year = new Date(prev.year);
                newDates.year.setFullYear(prev.year.getFullYear() + value);
            }

            return newDates;
        });
    };

    const openCalendar = () => {
        const selectedDate = prompt("Enter a date (YYYY-MM-DD):");
        if (selectedDate) {
            const parsedDate = new Date(selectedDate);
            if (!isNaN(parsedDate.getTime())) {
                setDates((prev) => ({ ...prev, [route]: parsedDate }));
            } else {
                alert("Invalid date format!");
            }
        }
    };

    return (
        <div className="mt-20">
            <div className="flex gap-10 items-center">
                <button className="text-5xl cursor-pointer text-zinc-600" onClick={() => changeDate(-1)}>
                    &lt;
                </button>
                <div className="flex flex-col items-center cursor-pointer" onClick={openCalendar}>
                    {route === "day" && (
                        <>
                            <h1 className="text-4xl text-gray-300">{formatDay(dates.day)}</h1>
                            <h1 className="text-gray-400">{formatDate(dates.day)}</h1>
                        </>
                    )}
                    {route === "week" && <h1 className="text-4xl text-gray-300">{getWeekRange(dates.week)}</h1>}
                    {route === "month" && <h1 className="text-4xl text-gray-300">{dates.month.toLocaleDateString("en-US", { month: "short", year: "numeric" })}</h1>}
                    {route === "year" && <h1 className="text-4xl text-gray-300">{dates.year.getFullYear()}</h1>}
                </div>
                <button className="text-5xl cursor-pointer text-zinc-600" onClick={() => changeDate(1)}>
                    &gt;
                </button>
            </div>
        </div>
    );
}

export default Header;

import React, { useEffect, useState } from 'react'
import Header from './Header'
import TaskSection from './TaskSection'
import { invoke } from '@tauri-apps/api/core';

function Content({ route }) {
  const formatDate = (date) => {
    return date.toLocaleDateString("en-US", { month: "short", day: "2-digit", year: "numeric" });
  };

  const [selectedDate, setSelectedDate] = useState(() => formatDate(new Date()));
  const [tasks, setTasks] = useState([]);
  
  const handleDateChange = (newDate) => {
    setSelectedDate(newDate);
  };

  const getTasks = async () => {
    try {
      console.log("tasks")
      await invoke("debug_terminal", { variable: "get task for", value: {field: route, date: selectedDate} });
      const res = await invoke("get_task", {field: route, date: selectedDate});
      setTasks(res);
    } catch (e) {
      setTasks([])
      await invoke("debug_terminal", { variable: 'error', value: e });
    }
  }
  useEffect(() => {
    getTasks();
  }, [route, selectedDate])

  const addTask = async (task) => {
    try {
      await invoke("debug_terminal", { variable: "add task", value: {header: route ,date: selectedDate, task} });
      await invoke("add_task", { field: route, date: selectedDate, task });
      await getTasks();
      // const res = await invoke("add_task", { field: route, date: selectedDate, task });
    } catch (e) {
      await invoke("debug_terminal", { variable: 'error', value: e });
    }
  }

  const deleteTask = async (task) => {
    try {
      await invoke("delete_task", {field: route, date: selectedDate, task});
      await getTasks();
    } catch (e) {
      await invoke("debug_terminal", { variable: 'error', value: e });
    }
  }

  return (
    <div className="flex flex-col items-center justify-center gap-10 ">
      <Header route={route} onDateChange={handleDateChange}/>
      <TaskSection addTask={addTask} tasks={tasks} deleteTask={deleteTask}/>
    </div>
  )
}

export default Content
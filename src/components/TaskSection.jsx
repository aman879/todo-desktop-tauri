import React, { useState } from 'react'
import SearchBar from './SearchBar'
import TasksLists from './TasksLists'

function TaskSection({addTask, tasks, deleteTask}) {

  return (
    <div className='w-full px-7'>
        <SearchBar addTask={addTask}/>
        <TasksLists tasks={tasks} deleteTask={deleteTask}/>
    </div>
  )
}

export default TaskSection
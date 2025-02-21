import React from 'react'
import dl from "../assets/delete.svg";

function TasksLists({tasks, deleteTask}) {
    console.log(tasks)
  return (
    <div className='flex flex-col text-xl text-gray-300 '>
        {tasks.map((item, index) =>
          <div key={index} className="flex flex-row justify-between shadow-lg shadow-blue-500/5  m-2 p-5 text-gray-100">
            <h1 className="capitalize">{item}</h1>
            <img src={dl} className="w-5 h-5 cursor-pointer" onClick={() => deleteTask(item)}/>
          </div>
        )}
    </div>
  )
}

export default TasksLists
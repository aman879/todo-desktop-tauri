import React from 'react'

function Nav({onRouteChange, route}) {

  return (
    <nav className='navbar border-b-2  ml-[-18px] mr-[-18px] border-zinc-900'>
        <ul className="text-gray-500 flex gap-10 items-center justify-center tracking-tight">
        {["day", "week", "month", "year"].map((item,index) => 
          <li 
          key={index} 
          onClick={() => onRouteChange(item)} 
          className={`capitalize cursor-pointer relative ${route === item ? "text-gray-200" : ""}`}
        >
          {item}
          <span 
            className={`absolute bottom-[-1px] left-1/2 -translate-x-1/2 w-[55px] h-[2px] 
                        transition-all duration-200 
                        ${route === item ? "bg-indigo-300 " : "bg-transparent"}`}
          />
        </li>
        
        )}
        </ul>
    </nav>
  )
}

export default Nav
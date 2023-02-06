const test = () => {
	
	let obj = {
		"nooe" : "okda"
	}
	
	let arr = ["reddd",'bleeee']
	
	const str = "nice"
	const cb = () => {}
	
	return (
		
			
			
			
		createElement("div", { id:"osx",...obj, }, "\n\t\t\t", 
			createElement(Nav, { ...obj, }, ), 
			createElement("button", { id:str, }, "odd ", ...arr, " money"), 
			createElement("input", { id:str, }, ), 
			createElement("listener", { onclick:()=>{cb}, }, ))
	)
}

function Nav({props}) {
	return (
		createElement("nav", { ...props, }, )
	)
}

test()
use std::{
	cell::RefCell,
	rc::{Rc,Weak},
};
//~ Esta linea es para poder ver la estructura con {:?} en println!
#[derive(Debug)]
//~ Estructura publica Node que recibe un elemento generico Copy
pub struct Node<T: Copy>{
	//~ Declaracion del elemento value de tipo T
	pub value: T,
	pub next: Option<Rc<RefCell<Node<T>>>>,
	pub prev: Option<Weak<RefCell<Node<T>>>>,
}

//~ Implementacion de la funcion new a la estructura Node
impl<T: Copy> Node<T>{
	//~ Se crea la funcion new
	pub fn new (value: T) -> Self{
		//~ Se le asignan los valores al nodo
		Node{
			value,
			next: None,
			prev: None,
		}
	}
}

impl<T: Copy> From <Node<T>> for Option <Rc<RefCell<Node<T>>>>{
	fn from(node: Node<T>) -> Self{
		Some(Rc::new(RefCell::new(node)))
	}
}

type NodePtr<T> = Rc<RefCell<Node<T>>>;

//~ Se crea una estructura List con la cabeza y la cola de la lista, siendo de tipo option y nodePtr
pub struct List<T: Copy>{
	head: Option<NodePtr<T>>,
	tail: Option<NodePtr<T>>,
}

//~ Se implementan funciones para la estructura List
impl<T: Copy> List<T>{
	//~ Se crea la función new para definir en la lista la cabeza y la cola con None
	pub fn new() -> Self{
		List{
			head: None,
			tail: None
		}
	}
	
	//~ funcion para agregaar a la lista en la parte de enfrente. Se recibe de parametro sus atributos y el valor a agregar
	pub fn push_front(&mut self,value: T){
		//~ Se crea un nuevo nodo donde se agregará al inicio de l lista
		let mut node = Node::new(value);
		
		//~ Con Option , se utiliza match para ver si un trae algo o no con none y some
		match &mut self.head.take(){
			//~ Si el atributo cabeza no trae nada, ejecuta el codigo siguiente
			None =>{
				//~ La funcion into es una conversion valor  valor que consume un valor de entrada
				self.head = node.into();
				//~ La funcion clone clona el nodo, entonces clona la cabeza y la deja en la cola
				self.tail = self.head.clone();
			},
			//~ Si trae algo, ejecuta el codigo siguiente
			Some(current_head) =>{
				//~ Si trae algo , clona la cabeza actual de la lista y lo agrega al atributo del nodo next
				node.next = Some(current_head.clone());
				//~ La funcion into es una conversion valor  valor que consume un valor de entrada
				self.head = node.into();
				//~ se guarda la direccion de memoria de la cabeza y la guarda en h, si hay algo ejecuta el codigo siguiente
				if let Some(h) = &self.head {
					//~ borrow_mut presta un valor envuelto . Este valor prestado no puede tomarse por otro hasta que lo suelte.
					 //~ Rc::downgrade se usa para hacer referencias ligeras o debiles. Son punteros debiles.
					current_head.borrow_mut().prev = Some(Rc::downgrade(&h));
				}
			}
		}
	}
	//~ funcion para agregar a la lista en la parte de atras. Se recibe de parametro sus atributos y el valor a agregar
	pub fn push_back(&mut self, value: T){
		//~ Se crea un nuevo nodo donde se agregará al inicio de la lista
		let mut node = Node:: new(value);
		
		//~ Con Option , se utiliza match para ver si un trae algo o no con none y some
		match &mut self.tail.take(){
			None =>{
				//~ La funcion into es una conversion valor  valor que consume un valor de entrada
				self.head = node.into();
				//~ La funcion clone clona el nodo, entonces clona la cabeza y la deja en la cola
				self.tail = self.head.clone();
			},
			//~ Si trae algo, ejecuta el codigo siguiente
			Some(current_tail) => {
				//~ Rc::downgrade se usa para hacer referencias ligeras o debiles. Son punteros debiles.
				node.prev = Some(Rc::downgrade(&current_tail));
				//~ La funcion into es una conversion valor  valor que consume un valor de entrada
				self.tail = node.into();
				//~ borrow_mut presta un valor envuelto . Este valor prestado no puede tomarse por otro hasta que lo suelte.
				//~ clona la cola actual de la lista y lo agrega al atributo del nodo next
				current_tail.borrow_mut().next = self.tail.clone();
			}
		}
	}
	//~ funcion para sacar de la lista en la parte de atras. Se recibe de parametro sus atributos 
	pub fn pop_back(&mut self) -> Option<T>{
		//~ Con Option , se utiliza match para ver si un trae algo o no con none y some.  
		match &mut self.tail.take(){
			//~ Si la cola no contiene nada, no hace nada
			None => None,
			//~ Si tieene cola realiza lo siguiente
			Some(tail) =>{
				//~ borrow_mut presta un valor envuelto . Este valor prestado no puede tomarse por otro hasta que lo suelte.
				let mut tail = tail.borrow_mut();
				
				let prev = tail.prev.take();
				match prev {
					None => {
						self.head.take();
					},
					Some(prev) => {
						let prev = prev.upgrade();
						if let Some(prev) = prev{
							prev.borrow_mut().next = None;
							self.tail = Some(prev);
						}
					}
				};
				
				Some(tail.value)
			}
		}
	}
	
	pub fn pop_front(&mut self) -> Option<T>{
		match &mut self.head.take(){
			None => None,
			Some(head) =>{
				let mut head = head.borrow_mut();
				let next = head.next.take();
				match next{
					None => {
						self.tail.take();
					},
					Some(next) =>{
						next.borrow_mut().prev = None;
						self.head = Some(next);
					}
				};
				
				Some(head.value)
			}
		}
	}
	
	pub fn empty(&mut self) -> bool{
		let vacio_Cabeza: bool;
		let vacio_Cola: bool;
		match &mut self.head.take(){
			None =>{
				vacio_Cabeza=true;
			},
			Some(next) =>{
				vacio_Cabeza=false;
			}
			
		}
		match &mut self.tail.take(){
			None =>{
				vacio_Cola=true;
			},
			Some(prev) =>{
				vacio_Cola=false;
			}
			
		}
		return vacio_Cabeza && vacio_Cola;
	}
	
	//~ pub fn size(&mut self) -> i32{
		
	//~ }
}


fn main(){
	
	let mut list = List::new();
		list.push_back(1);
		//~ list.push_back(2);
		//~ list.push_back(3);
		//~ list.push_back(4);
		
		
		
		let mut node = Node::new(list.pop_back());
		//~ match node.value{
			//~ None => {
				
			//~ },
			//~ Some (value)=> {
				//~ let numero = value;
				//~ println!("nodo {}",numero);
			//~ }
		//~ }
		
		//~ node = Node:: new(list.pop_back());
		//~ println!("nodo {:?}",node);
		let vacio = list.empty();
		println!("nodo {}",vacio);
}
//~ #[cfg(test)]
//~ mod tests{
	//~ use super::*;
	
	//~ #[test]
	//~ fn works_builds_list(){
		//~ let mut list = List::new();
		//~ list.push_back(1);
		//~ list.push_back(2);
		//~ list.push_back(3);
		//~ list.push_back(4);
		
		//~ assert_eq!(list.pop_back(), Some(4));
		//~ assert_eq!(list.pop_back(), Some(3));
		//~ assert_eq!(list.pop_back(), Some(2));
		//~ assert_eq!(list.pop_back(), Some(1));
		//~ assert_eq!(list.pop_back(), None);
		
	//~ }
	
	//~ #[test]
	//~ fn works_builds_list_front(){
		//~ let mut list = List::new();
		//~ list.push_front(1);
		//~ list.push_front(2);
		//~ list.push_front(3);
		//~ list.push_front(4);
		
		//~ assert_eq!(list.pop_front(), Some(4));
		//~ assert_eq!(list.pop_front(), Some(3));
		//~ assert_eq!(list.pop_front(), Some(2));
		//~ assert_eq!(list.pop_front(), Some(1));
		//~ assert_eq!(list.pop_front(), None);
	//~ }
//~ }

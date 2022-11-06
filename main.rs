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
	
	
    //~ Omite los warnigs por variables sin usar
    #[allow(unused_variables)]
	//~ empty(), verifica si la lista está vacía, devolviendo verdadero de ser así.
	pub fn empty(&mut self) -> bool{
		let vacio_cabeza: bool;
		let vacio_cola: bool;
		match &mut self.head.take(){
			None =>{
				vacio_cabeza=true;
			},
			Some(next) =>{
				vacio_cabeza=false;
			}
			
		}
		match &mut self.tail.take(){
			None =>{
				vacio_cola=true;
			},
			Some(prev) =>{
				vacio_cola=false;
			}
			
		}
		return vacio_cabeza && vacio_cola;
	}
	

	//~ Funcion size(), devuelve el tamaño de la lista en entero
    pub fn size(&mut self) -> i32{
		//~ tam_lista guarda el tamanio de la lista 
        let mut tam_lista = 0; 
		//~ fin_lista sera verdade una vez que se recorra toda la lista
        let mut fin_lista:bool = false;

		//~ El valor de head de la lista para empezar desde el primer nodo
        match &mut self.head.take(){
			//~ Si no encuentra el valor de head quiere decir que la lista esta vacia
            None => {
                tam_lista = 0;
            },
			//~ En caso de encotrar un valor, la lista ya como minimo tiene un nodo
            Some(head) => {
                //~ incrementa indicando que se encontro el primer nodo
                tam_lista = tam_lista + 1;

                //~ Recupera el valor envuelto (wrappeed) de head
                let mut head = head.borrow_mut();
                //~ Toma el siguiente valor de head para pasar al siguiente nodo
                let mut next = head.next.take();
                
				//~ Mientra no termine de recorrer toda la lista
                while fin_lista != true{
					//~ Revisa si se encontro un nodo siguiente
                    match next{
						//~ Si no se encuentra, significa que ya recorrio toda la lista y termina
                        None => {
                            fin_lista = true;
                        },
						//~ Si encuentra un valor, hay otro nodo
                        Some(new_head) => {
                            //~ Incrementa el contador del tamanio de la lista
                            tam_lista = tam_lista + 1;

							//~ Toma el siguiente valor de head para pasar al siguiente nodo
                            let mut head = new_head.borrow_mut();
                            next = head.next.take();
                        }
                    }
					//~ Se repite el proceso hasta llegar al final de la lista
                }
            }
        }
		//~ Regresa el tamanio de la lista
        return tam_lista;
    }


    //~ clear(), elimina todos los nodos de la lista.
    pub fn clear(&mut self){
        //~ vacio sera verdadero cuando la lista quede vacia
        let mut vacio:bool = false;

        //~ Mientras la lista no este vacia
        while vacio != true {
            //~ Realiza un pop_back para eliminar el ultimo nodo y recibir el objeto eliminado
            match self.pop_back(){
                //~ Si ya no obtuvo nada, ya elimino todos los nodos
                None =>{
                    //~ Se cumple la condicion y se saldra del ciclo
                    vacio = true;
                },
                //~ Si regresa el nodo eliminado
                Some(_) =>{
                    //~ No hace nada
                    //~ Realizara otro pop_back en el siguiente ciclo
                }
            }
        }
    }
}


fn main(){
	
	//~ Se crea la lista
	let mut list = List::new();

	//~ Se ingresan algunos nodos
	list.push_front(1);
	list.push_front(2);
	list.push_back(3);
	list.push_back(4);
	list.push_back(5);
	 
	
	
	//~ tam_lista almacenara el tamanio de la lista y despues imprime su valor
	let mut tam_lista:i32 = list.size();
	println!("Hay {} elementos en la lista", tam_lista);
	
	//~ Se vacia la lista
	list.clear();
	println!("\nSe vacio la lista");

	tam_lista = list.size();
	println!("\nDespues del clear hay {} elementos en la lista", tam_lista);


	//~ Verificamos si la lista esta vacia
	let vacio = list.empty();
	if vacio == true{
		println!("\nLa lista esta vacía\n");
	}else{
		println!("\nLa lista NO esta vacía\n");
	}

	//~ let mut node = Node::new(list.pop_back());
		
		//let mut node = Node::new(list.pop_back());
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

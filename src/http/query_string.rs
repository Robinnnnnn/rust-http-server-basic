use std::collections::{HashMap};

/*

This struct takes a query string and stores all 
the parameters as key value pairs in a hash map 

*/
#[derive(Debug)]
pub struct QueryString<'buf> {
    data: HashMap<&'buf str, Value<'buf>>
}

/*

this enum classifies whether or not the parameters have a single or multiple values

 */
#[derive(Debug)]
pub enum Value<'buf> {
    Single(&'buf str),
    Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(key)
    }
}

/*

    We dont use FromStr to convert here because FromStr does not accept lifetimes

 */

impl<'buf> From<&'buf str> for QueryString<'buf> {
    fn from(s: &'buf str) -> Self{

        let mut data = HashMap::new();

        //seperating query string into parameter strings
        for sub_str in s.split('&') {
            let mut key = sub_str;
            let mut val = "";
            //seperating parameter strings into key:value pairs
            if let Some(i) = sub_str.find('=') {
                key = &sub_str[..i];
                val = &sub_str[i+1..];
            }

            
            /*
                adding the key value pairs to the hashmap.

                if no matching key, create a key and insert a value

                if matching key, and key enum type is single,
                dereference, and change enum type to multiple and insert the values as a vector

                if matching and key enum type is multiple, push value to vector


             */
            data.entry(key)
                .and_modify(|existing: &mut Value| match existing {
                    Value::Single(prev_value) => {
                        *existing = Value::Multiple(vec![prev_value, val]);
                    },
                    Value::Multiple(vec) => vec.push(val) ,
                })
                .or_insert(Value::Single(val));

        }

        QueryString{data};
        unimplemented!()
    }


}
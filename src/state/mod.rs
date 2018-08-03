
macro_rules! create_state {
    ( $name:ident { $first:ident, $( $state:ident ),* } ) => {

        #[derive(Debug, PartialEq, Clone, Copy)]
        enum $name {
            $first,
            $( $state ),*
        }

        impl $name {
            #[allow(unused)]
            fn next(self) -> Option<Self> {
                let mut cmp = $name::$first;
                $(
                    if self == cmp {
                        return Some($name::$state);
                    }
                    cmp = $name::$state;
                )*
                return None;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    create_state! {
        TestState {
            Start, Next, End
        }
    }

    #[test]
    fn test_next() {
        assert_eq!(Some(TestState::Next), TestState::Start.next());
        assert_eq!(Some(TestState::End), TestState::Next.next());
        assert_eq!(None, TestState::End.next());
    }
}

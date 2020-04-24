# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-parser/index.test.ts --update-snapshots` to update.

## `experimental > dynamic-import > direct-calls-only`

```javascript
Program {
  comments: Array []
  corrupt: false
  directives: Array []
  filename: 'input.js'
  hasHoistedVars: false
  interpreter: undefined
  mtime: undefined
  sourceType: 'script'
  syntax: Array []
  loc: Object {
    filename: 'input.js'
    end: Object {
      column: 0
      index: 50
      line: 4
    }
    start: Object {
      column: 0
      index: 0
      line: 1
    }
  }
  diagnostics: Array [
    Object {
      origins: Array [Object {category: 'js-parser'}]
      description: Object {
        category: 'parse/js'
        message: PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE {value: 'The only valid meta property for import is import.meta'}
      }
      location: Object {
        filename: 'input.js'
        mtime: undefined
        sourceType: 'script'
        end: Object {
          column: 20
          index: 44
          line: 2
        }
        start: Object {
          column: 16
          index: 40
          line: 2
        }
      }
    }
  ]
  body: Array [
    FunctionDeclaration {
      id: BindingIdentifier {
        name: 'failsParse'
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 19
            index: 19
            line: 1
          }
          start: Object {
            column: 9
            index: 9
            line: 1
          }
        }
      }
      loc: Object {
        filename: 'input.js'
        end: Object {
          column: 1
          index: 49
          line: 3
        }
        start: Object {
          column: 0
          index: 0
          line: 1
        }
      }
      head: FunctionHead {
        async: false
        generator: false
        hasHoistedVars: false
        params: Array []
        predicate: undefined
        rest: undefined
        returnType: undefined
        thisType: undefined
        typeParameters: undefined
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 22
            index: 22
            line: 1
          }
          start: Object {
            column: 19
            index: 19
            line: 1
          }
        }
      }
      body: BlockStatement {
        directives: Array []
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 1
            index: 49
            line: 3
          }
          start: Object {
            column: 22
            index: 22
            line: 1
          }
        }
        body: Array [
          ReturnStatement {
            loc: Object {
              filename: 'input.js'
              end: Object {
                column: 23
                index: 47
                line: 2
              }
              start: Object {
                column: 2
                index: 26
                line: 2
              }
            }
            argument: CallExpression {
              arguments: Array []
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 22
                  index: 46
                  line: 2
                }
                start: Object {
                  column: 9
                  index: 33
                  line: 2
                }
              }
              callee: MetaProperty {
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 20
                    index: 44
                    line: 2
                  }
                  start: Object {
                    column: 9
                    index: 33
                    line: 2
                  }
                }
                meta: Identifier {
                  name: 'import'
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 15
                      index: 39
                      line: 2
                    }
                    start: Object {
                      column: 9
                      index: 33
                      line: 2
                    }
                  }
                }
                property: Identifier {
                  name: 'then'
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 20
                      index: 44
                      line: 2
                    }
                    start: Object {
                      column: 16
                      index: 40
                      line: 2
                    }
                  }
                }
              }
            }
          }
        ]
      }
    }
  ]
}
```
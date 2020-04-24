# `index.test.ts`

**DO NOT MODIFY**. This file has been autogenerated. Run `rome test packages/@romejs/js-parser/index.test.ts --update-snapshots` to update.

## `esprima > es2015-generator > generator-method-with-invalid-computed-name`

```javascript
Program {
  comments: Array []
  corrupt: true
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
      index: 25
      line: 2
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
      location: Object {
        filename: 'input.js'
        mtime: undefined
        sourceType: 'script'
        end: Object {
          column: 4
          index: 4
          line: 1
        }
        start: Object {
          column: 4
          index: 4
          line: 1
        }
      }
      description: Object {
        category: 'parse/js'
        message: PARTIAL_BLESSED_DIAGNOSTIC_MESSAGE {value: 'Unclosed property name'}
        advice: Array [
          log {
            category: 'info'
            message: 'We expected to find the closing character <emphasis>]</emphasis> here'
          }
          frame {
            location: Object {
              filename: 'input.js'
              end: Object {
                column: 11
                index: 11
                line: 1
              }
              start: Object {
                column: 11
                index: 11
                line: 1
              }
            }
          }
        ]
      }
    }
  ]
  body: Array [
    ExpressionStatement {
      loc: Object {
        filename: 'input.js'
        end: Object {
          column: 24
          index: 24
          line: 1
        }
        start: Object {
          column: 0
          index: 0
          line: 1
        }
      }
      expression: ObjectExpression {
        loc: Object {
          filename: 'input.js'
          end: Object {
            column: 23
            index: 23
            line: 1
          }
          start: Object {
            column: 1
            index: 1
            line: 1
          }
        }
        properties: Array [
          ObjectMethod {
            kind: 'method'
            key: ComputedPropertyKey {
              value: ReferenceIdentifier {
                name: 'yield'
                loc: Object {
                  filename: 'input.js'
                  end: Object {
                    column: 10
                    index: 10
                    line: 1
                  }
                  start: Object {
                    column: 5
                    index: 5
                    line: 1
                  }
                }
              }
              variance: undefined
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 10
                  index: 10
                  line: 1
                }
                start: Object {
                  column: 4
                  index: 4
                  line: 1
                }
              }
            }
            loc: Object {
              filename: 'input.js'
              end: Object {
                column: 23
                index: 23
                line: 1
              }
              start: Object {
                column: 3
                index: 3
                line: 1
              }
            }
            head: FunctionHead {
              async: false
              generator: true
              hasHoistedVars: false
              predicate: undefined
              rest: undefined
              returnType: undefined
              thisType: undefined
              typeParameters: undefined
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 15
                  index: 15
                  line: 1
                }
                start: Object {
                  column: 11
                  index: 11
                  line: 1
                }
              }
              params: Array [
                BindingIdentifier {
                  name: 'iter'
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 15
                      index: 15
                      line: 1
                    }
                    start: Object {
                      column: 11
                      index: 11
                      line: 1
                    }
                  }
                  meta: PatternMeta {
                    optional: undefined
                    typeAnnotation: undefined
                    loc: Object {
                      filename: 'input.js'
                      end: Object {
                        column: 15
                        index: 15
                        line: 1
                      }
                      start: Object {
                        column: 11
                        index: 11
                        line: 1
                      }
                    }
                  }
                }
              ]
            }
            body: BlockStatement {
              directives: Array []
              loc: Object {
                filename: 'input.js'
                end: Object {
                  column: 23
                  index: 23
                  line: 1
                }
                start: Object {
                  column: 15
                  index: 15
                  line: 1
                }
              }
              body: Array [
                ExpressionStatement {
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 18
                      index: 18
                      line: 1
                    }
                    start: Object {
                      column: 15
                      index: 15
                      line: 1
                    }
                  }
                  expression: CallExpression {
                    arguments: Array []
                    loc: Object {
                      filename: 'input.js'
                      end: Object {
                        column: 18
                        index: 18
                        line: 1
                      }
                      start: Object {
                        column: 15
                        index: 15
                        line: 1
                      }
                    }
                    callee: ReferenceIdentifier {
                      name: 'INVALID_PLACEHOLDER'
                      loc: Object {
                        filename: 'input.js'
                        end: Object {
                          column: 16
                          index: 16
                          line: 1
                        }
                        start: Object {
                          column: 15
                          index: 15
                          line: 1
                        }
                      }
                    }
                  }
                }
                BlockStatement {
                  body: Array []
                  directives: Array []
                  loc: Object {
                    filename: 'input.js'
                    end: Object {
                      column: 21
                      index: 21
                      line: 1
                    }
                    start: Object {
                      column: 19
                      index: 19
                      line: 1
                    }
                  }
                }
              ]
            }
          }
        ]
      }
    }
  ]
}
```
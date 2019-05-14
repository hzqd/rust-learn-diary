//rust--�ʺŲ�����
// ʲô���ʺŲ�����?
// �ο�: https://doc.rust-lang.org/book/second-edition/ch09-02-recoverable-errors-with-result.html
// �ο�: https://stackoverflow.com/questions/42917566/what-is-this-question-mark-operator-about


// ����Rust��û��Exception�쳣������﷨,
// Rustֻ��panic����, ����panic����������, ��Ϊû���ṩ try �����﷨.
// Rust���쳣������ͨ�� Result �� Ok �� Err ��Ա�����ݺͰ���������Ϣ.
// Ȼ��������Ϣ�Ĵ���һ�㶼��Ҫͨ��match�������ͽ��бȽ�, ���Ժܶ�ʱ��
// ����Ƚ�����, ͨ��?��������Ok��Err���ж�.


// ����������ṩ��һ����ʹ��?���� �Լ� һ��ʹ��?���ŵ���������.
fn halves_if_even<'a >(i: i32) -> Result<i32, &'a str> {                       // ȡ��ֵ�Ķ���֮һ.
    if i % 2 == 0 {
        Ok(i/2)
    } else {
        Err("error")
    }
}

fn not_use_question_mark() {
    let a = 10;                                                                // ������ĳ� 9 �ͻᱨ��.
    let half = halves_if_even(a);
    let half = match half {
        Ok(item) => item,
        Err(e) => panic!(e),
    };
    assert_eq!(half, 5);
}


fn use_question_mark<'a >() -> Result<i32, &'a str> {                          // �������Ҫ����Result
    let a = 10;
    let half = halves_if_even(a)?;                                             // ��Ϊ?Ҫ�������ڵĺ�������Ҫ����Result
    assert_eq!(half, 5);
    Ok(half)                                                                   
}


fn main() {
    not_use_question_mark();
    let _ = use_question_mark();
}
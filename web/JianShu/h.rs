//rust--ʲô��ownership?
// ʲô������Ȩ?
// 1. Each value in Rust has a variable that��s called its owner.
// 2. There can only be one owner at a time.
// 3. When the owner goes out of scope, the value will be dropped.

fn print(msg: String) {
    println!("{}", msg)
}


// ���ݸ�ֵ �� ����Ȩת��
fn create_a_variable() {
    // system �Ǳ�����, "hello" ������
    // ����"hello"�洢���ڴ���.
    // ������ system  ӵ������ "hello" ������Ȩ.
    let system = String::from("windows");

    // ����������������, Ĭ������»�ת������Ȩ,
    // ���ұ������ᱻ���յ�, �����������ٵ������������.
    print(system);
    // print(system);      // ����ᱨ��: value used here after move

    {
        let a = "hello";
    }

    println!("{}", a)     // ����ᱨ��: cannot find value `a` in this scope

    // ��ע:
    // ��������еĴ���, չʾ������Ȩ�����й���.
    // let system = String::from("windows") ��Ӧ���ǵ�һ������.
    //
    // print(system) ��Ӧ���ǵڶ�������, ����ͬһʱ��ֻ��ӵ��һ������Ȩ,
    // Ҳ����˵���������������ݽ�ȥ֮��, system�Ͳ���ӵ�� "windows" ������ݵ�����Ȩ,
    // ��һ��������û������Ȩʱ, ���������Զ����ոñ�����.
    //
    // println!("{}", a) ��Ӧ���ǵ���������, ��������һ�����������еı�����,
    // ���������еı�������û�д��ݳ���, ��˱�������drop�����������.
}


fn main() {
    create_a_variable()
}